# 3Yiddish benchmark

How good is it? Honest numbers.

Not the best yet. We tested seven Yiddish speech-to-text models on 25 takes of free heimish
speech (1,191 words): two paid online services, 3Yiddish, the two ivrit-ai Yiddish models,
and plain Whisper as it comes from OpenAI.

**Score out of 100**

| Yiddish Labs | Yiddish Tools | 3Yiddish | ivrit-ai large-v3 | ivrit-ai turbo | Whisper large-v3 | Whisper turbo |
|---|---|---|---|---|---|---|
| **81.5** | 64.4 | 54.5 | 53.0 | 48.3 | 8.2 | 0.0 |

The score is the plain average of three parts: words right, the place in each take (best
gets full points, worst gets none), and the share of takes with nothing wrong.

**The details** (the best in each row is bold)

| | Yiddish Labs | Yiddish Tools | 3Yiddish | ivrit-ai large-v3 | ivrit-ai turbo | Whisper large-v3 | Whisper turbo |
|---|---|---|---|---|---|---|---|
| Wrong words per 100 | **2.9** | 4.7 | 12.6 | 18.3 | 15.6 | 92.9 | 110.4 |
| Placed best (of 25 takes) | **16** | 6 | 4 | 5 | 2 | 0 | 0 |
| Average place (1 is best, 7 is worst) | **1.52** | 2.08 | 2.92 | 3.56 | 3.84 | 5.96 | 7 |
| Takes with nothing wrong (of 25) | **14** | 4 | 2 | 5 | 2 | 0 | 0 |
| Loshn koydesh wrong (of 13 spots) | **1** | 2 | 12 | 11 | 10 | 13 | 13 |
| English words wrong (of 14 spots) | **0** | 3 | 14 | 8 | 13 | 14 | 14 |
| Names wrong (of 3 spots) | **0** | **0** | 3 | 2 | 1 | 3 | 3 |
| Wrong spots | **16** | 31 | 87 | 89 | 99 | - | - |
| Spelled its own way, but fine | 79 | 74 | 9 | 56 | 41 | - | - |
| Added words nobody said | 2 | **0** | 7 | 4 | 10 | - | - |
| Dropped words | **2** | 5 | **2** | 4 | 3 | - | - |

**How it was judged**

- One speaker read the transcripts of Yiddish Labs, Yiddish Tools and 3Yiddish without
  knowing which was which, ranked them in every take, and judged every word.
- The other four were added later. Where one of them wrote the same words he had already
  judged, his verdict stands. Everything else was judged by an AI agent following his
  earlier answers, and a second agent checked it. He has not gone over those yet.
- Counting only what he forgave himself, ivrit-ai large-v3 scores 51.9 and ivrit-ai
  turbo 47.3. The other scores stay the same.
- A model he did not rank is placed in each take by its wrong words. That rule matched his
  own order in 58 of 63 pairs.

**What to keep in mind**

- It's 25 takes from one speaker. More takes and more speakers are coming.
- Plain Whisper does not write heimish Yiddish. Large-v3 spells by rough sound and turbo
  writes German in Latin letters, so nearly every word counts as wrong, and a model can go
  over 100 wrong words per 100 when it also adds words. The last four rows have no meaning
  for them.
- "Spelled its own way, but fine" is not a mistake. It counts the spots where a transcript
  spelled something differently and that was fine too.
- The score depends on who else is in the table, because places are relative. With only
  the first three models the scores were 77.0, 55.8 and 38.5. Wrong words per 100 does not
  move when models are added.
- The answer key grew out of 3Yiddish's own draft, corrected by the speaker. That can
  help 3Yiddish and the ivrit-ai turbo model it was trained from where a wrong word was
  missed in the key.
- 3Yiddish and the ivrit-ai and Whisper models are free and run on your own computer.
  Yiddish Labs and Yiddish Tools are paid and run online.
- This is where we are, and it's the number every new model has to beat.
