# 3Yiddish benchmark

How good is it? Honest numbers.

Not the best yet. We tested it against two paid online services, Yiddish Labs and Yiddish
Tools, on 25 takes of free heimish speech (1,189 words). One speaker read all three
transcripts without knowing which was which, ranked them, and judged every word.

**Score out of 100**

| Yiddish Labs | Yiddish Tools | 3Yiddish |
|---|---|---|
| **77.0** | 55.8 | 38.5 |

The score is the plain average of three parts: words right, the blind ranking (2 points for
best, 1 for second, as a share of the most possible), and the share of takes with nothing
wrong.

**The details** (the best in each row is bold)

| | Yiddish Labs | Yiddish Tools | 3Yiddish |
|---|---|---|---|
| Wrong words per 100 | **2.9** | 4.7 | 12.6 |
| Ranked best (of 25 takes) | **16** | 6 | 4 |
| Ranked worst (of 25 takes) | **2** | 3 | 19 |
| Average place (1 is best) | **1.44** | 1.88 | 2.6 |
| Takes with nothing wrong (of 25) | **14** | 4 | 2 |
| Wrong spots (of 177 where they disagreed) | **16** | 31 | 87 |
| Spelled its own way, but fine | 79 | 74 | 9 |
| Loshn koydesh wrong (of 13 spots) | **1** | 2 | 12 |
| English words wrong (of 14 spots) | **0** | 3 | 14 |
| Names wrong (of 3 spots) | **0** | **0** | 3 |
| Wrote a different word | **12** | 25 | 78 |
| Added words nobody said | 2 | **0** | 7 |
| Dropped words | **2** | 5 | **2** |

What to keep in mind:
- It's 25 takes from one speaker. More takes and more speakers are coming.
- "Spelled its own way, but fine" is not a mistake. It counts the spots where a transcript
  spelled something differently and the speaker said that's fine too.
- 3Yiddish is free and runs on your own computer. The other two are paid and run online.
- This is where we are, and it's the number every new model has to beat.
