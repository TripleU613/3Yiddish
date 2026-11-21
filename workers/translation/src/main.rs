//! Native local NLLB translation; independent of the reasoning LLM.
use anyhow::{Context, Result, ensure};
use axum::{
    Json, Router,
    extract::{DefaultBodyLimit, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use clap::Parser;
use ct2rs_sys::{ComputeType, Config, TranslationOptions, Translator};
use sentencepiece::SentencePieceProcessor;
use serde::Deserialize;
use serde_json::{Value, json};
use std::{net::SocketAddr, path::PathBuf, sync::Arc, time::Instant};
use tokio::sync::Semaphore;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    model: PathBuf,
    #[arg(long, default_value = "127.0.0.1:8799")]
    listen: SocketAddr,
    #[arg(long, default_value_t = 4)]
    threads: usize,
}
struct Engine {
    translator: Translator,
    tokenizer: SentencePieceProcessor,
    permit: Arc<Semaphore>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Input {
    /// One text, answered as "text".
    #[serde(default)]
    text: Option<String>,
    /// Several texts (an answer's sentences), translated together and answered as "texts".
    #[serde(default)]
    texts: Option<Vec<String>>,
    direction: String,
}
impl Input {
    fn all(&self) -> Vec<&str> {
        match (&self.text, &self.texts) {
            (Some(t), None) => vec![t.as_str()],
            (None, Some(ts)) => ts.iter().map(String::as_str).collect(),
            _ => Vec::new(),
        }
    }
    fn valid(&self) -> bool {
        let all = self.all();
        !all.is_empty()
            && all.len() <= 32
            && all.iter().all(|t| !t.trim().is_empty())
            && all.iter().map(|t| t.chars().count()).sum::<usize>() <= 16000
    }
}
struct Error(StatusCode, String);
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (
            self.0,
            Json(json!({"error":{"message":self.1,"type":"translation_error"}})),
        )
            .into_response()
    }
}
fn languages(direction: &str) -> Result<(&'static str, &'static str)> {
    match direction {
        "en-yi" => Ok(("eng_Latn", "ydd_Hebr")),
        "yi-en" => Ok(("ydd_Hebr", "eng_Latn")),
        _ => anyhow::bail!("direction must be en-yi or yi-en"),
    }
}
impl Engine {
    fn translate(&self, input: Input) -> Result<Value> {
        let (source, target) = languages(&input.direction)?;
        ensure!(
            input.valid(),
            "Send 1..32 nonempty texts of up to 16000 characters in all"
        );
        let mut batch = Vec::new();
        for text in input.all() {
            let mut tokens = vec![source.to_owned()];
            tokens.extend(self.tokenizer.encode(text)?.into_iter().map(|p| p.piece));
            tokens.push("</s>".into());
            ensure!(
                tokens.len() <= 512,
                "Translation exceeds the 512-token source limit; shorten or split the message"
            );
            batch.push(tokens);
        }
        let source_tokens: usize = batch.iter().map(Vec::len).sum();
        let options: TranslationOptions<String, String> = TranslationOptions {
            beam_size: 4,
            max_input_length: 0,
            max_decoding_length: 512,
            return_end_token: true,
            ..Default::default()
        };
        let start = Instant::now();
        let prefixes = vec![vec![target]; batch.len()];
        let results = self
            .translator
            .translate_batch_with_target_prefix(&batch, &prefixes, &options, None)?;
        let mut texts = Vec::new();
        let mut target_tokens = 0;
        for result in &results {
            let output = result
                .hypotheses
                .first()
                .context("No translation returned")?;
            ensure!(
                output.last().is_some_and(|s| s == "</s>"),
                "Translation reached its output limit; shorten the message"
            );
            target_tokens += output.len();
            let pieces: Vec<_> = output
                .iter()
                .filter(|s| {
                    !matches!(
                        s.as_str(),
                        "eng_Latn" | "ydd_Hebr" | "</s>" | "<s>" | "<pad>"
                    )
                })
                .collect();
            let text = self.tokenizer.decode_pieces(&pieces)?;
            ensure!(!text.trim().is_empty(), "Translation was empty");
            texts.push(text);
        }
        ensure!(texts.len() == batch.len(), "A translation is missing");
        let mut answer = json!({"model":"nllb1.3b-heimish-int8","direction":input.direction,"finish_reason":"stop","elapsed_ms":start.elapsed().as_millis(),"usage":{"prompt_tokens":source_tokens,"completion_tokens":target_tokens},"license":"CC-BY-NC-4.0"});
        if input.texts.is_some() {
            answer["texts"] = json!(texts);
        } else {
            answer["text"] = json!(texts[0]);
        }
        Ok(answer)
    }
}
async fn translate(
    State(engine): State<Arc<Engine>>,
    Json(input): Json<Input>,
) -> Result<Json<Value>, Error> {
    if languages(&input.direction).is_err() || !input.valid() {
        return Err(Error(
            StatusCode::BAD_REQUEST,
            "Use text, or texts (up to 32), nonempty and up to 16000 characters in all, and direction en-yi or yi-en".into(),
        ));
    }
    let permit = engine.permit.clone().try_acquire_owned().map_err(|_| {
        Error(
            StatusCode::TOO_MANY_REQUESTS,
            "Translator is busy; retry shortly".into(),
        )
    })?;
    tokio::task::spawn_blocking(move || {
        let _permit = permit;
        engine.translate(input)
    })
    .await
    .map_err(|_| {
        Error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Translation task failed".into(),
        )
    })?
    .map(Json)
    .map_err(|e| Error(StatusCode::UNPROCESSABLE_ENTITY, e.to_string()))
}
#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    ensure!(
        args.listen.ip().is_loopback(),
        "Translation worker must bind loopback"
    );
    ensure!((1..=32).contains(&args.threads), "threads must be 1..32");
    let engine = Arc::new(Engine {
        translator: Translator::new(
            &args.model,
            &Config {
                compute_type: ComputeType::INT8,
                num_threads_per_replica: args.threads,
                max_queued_batches: 1,
                ..Default::default()
            },
        )?,
        tokenizer: SentencePieceProcessor::open(args.model.join("sentencepiece.bpe.model"))?,
        permit: Arc::new(Semaphore::new(1)),
    });
    let app=Router::new().route("/healthz",get(||async{Json(json!({"status":"ready","model":"nllb1.3b-heimish-int8","license":"CC-BY-NC-4.0","runtime":"Rust/CTranslate2"}))})).route("/v1/translate",post(translate)).layer(DefaultBodyLimit::max(128*1024)).with_state(engine);
    let listener = tokio::net::TcpListener::bind(args.listen).await?;
    eprintln!(
        "Native translation ready on {}; request text is not logged",
        args.listen
    );
    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            let _ = tokio::signal::ctrl_c().await;
        })
        .await?;
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn yiddish_uses_its_own_script_language_tag() {
        assert_eq!(languages("en-yi").unwrap(), ("eng_Latn", "ydd_Hebr"));
        assert_eq!(languages("yi-en").unwrap(), ("ydd_Hebr", "eng_Latn"));
        assert!(languages("he-yi").is_err());
    }
    #[test]
    fn one_text_or_several_are_taken_but_not_both() {
        let input = |v: Value| serde_json::from_value::<Input>(v).unwrap();
        assert!(input(json!({"text":"a","direction":"en-yi"})).valid());
        assert!(input(json!({"texts":["a","b"],"direction":"en-yi"})).valid());
        assert!(!input(json!({"text":"a","texts":["b"],"direction":"en-yi"})).valid());
        assert!(!input(json!({"texts":[],"direction":"en-yi"})).valid());
        assert!(!input(json!({"texts":["a",""],"direction":"en-yi"})).valid());
        assert!(!input(json!({"texts":vec!["a"; 33],"direction":"en-yi"})).valid());
    }
}
