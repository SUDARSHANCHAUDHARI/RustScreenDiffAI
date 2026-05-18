# RustScreenDiffAI

Rust CLI pixel diff engine. Compares two screenshots pixel-by-pixel and reports visual regressions with a Pass/Fail verdict.

## Install

```bash
cargo build --release
# binary at target/release/screendiff
```

## Usage

```bash
# Compare two screenshots (default threshold: 1%)
screendiff compare before.png after.png

# Custom threshold (5% allowed diff)
screendiff compare before.png after.png --threshold 0.05

# JSON output
screendiff compare before.png after.png --json
```

## How it works

1. Loads both images with the `image` crate
2. Verifies dimensions match — errors if they differ
3. Compares every pixel pair
4. Calculates `diff_percent = diff_pixels / total_pixels`
5. Verdict: **Pass** if `diff_percent ≤ threshold`, **Fail** otherwise

## Output

```
ScreenDiff Report
Before:       before.png
After:        after.png
Total Pixels: 10000
Diff Pixels:  42
Diff:         0.42%
Threshold:    1.00%
Verdict:      PASS
```

## Verdicts

| Verdict | Condition |
|---|---|
| `PASS` | Diff within threshold |
| `FAIL` | Diff exceeds threshold |

## Test

```bash
cargo test
```

11 integration tests — zero diff, full diff, single pixel, dimension mismatch, threshold, CLI.

## Stack

Rust · clap · image · serde · colored · anyhow
