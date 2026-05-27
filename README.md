# RustScreenDiffAI

![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange?logo=rust)
![License](https://img.shields.io/badge/License-MIT-blue)

RustScreenDiffAI is a Rust CLI pixel-diff engine for screenshot regression checks. It compares two images pixel by pixel, calculates the changed-pixel percentage, and returns a clear `PASS` or `FAIL` verdict against a configurable threshold.

## Why This Exists

Visual regressions are easiest to miss when screens look “mostly fine.” RustScreenDiffAI provides a deterministic, script-friendly image comparison step that can be used in QA workflows, CI jobs, screenshot tests, and digital signage review pipelines.

## Features

- Compares two PNG or image files using the Rust `image` crate.
- Validates that both screenshots have identical dimensions.
- Counts changed pixels exactly.
- Calculates diff percentage from total pixels.
- Supports configurable pass/fail threshold.
- Emits terminal output for manual review.
- Emits JSON output for automation.
- Handles identical images, partial diffs, full diffs, and dimension mismatch errors.

## Installation

```bash
git clone https://github.com/SUDARSHANCHAUDHARI/RustScreenDiffAI.git
cd RustScreenDiffAI
cargo build --release
```

The binary is created at:

```bash
target/release/screendiff
```

Optional local install:

```bash
cargo install --path .
```

## Usage

```bash
# Compare two screenshots with the default 1% threshold
screendiff compare before.png after.png

# Allow up to 5% difference
screendiff compare before.png after.png --threshold 0.05

# Emit JSON
screendiff compare before.png after.png --json
```

## Included Example

The repository includes two tiny PPM fixtures:

```bash
screendiff compare examples/before.ppm examples/after.ppm --threshold 0.10
```

Real output:

```text
ScreenDiff Report
Before: examples/before.ppm
After: examples/after.ppm
Total Pixels: 16
Diff Pixels: 1
Diff: 6.25%
Threshold: 10.00%
Verdict: PASS
```

## How It Works

1. Load both images.
2. Verify dimensions match.
3. Compare every pixel pair.
4. Count changed pixels.
5. Calculate `diff_percent = diff_pixels / total_pixels`.
6. Return `PASS` when `diff_percent <= threshold`; otherwise return `FAIL`.

## Example Output

```text
ScreenDiff Report
Before:        before.png
After:         after.png
Total Pixels:  10000
Diff Pixels:   42
Diff:          0.42%
Threshold:     1.00%
Verdict:       PASS
```

## Threshold Guidance

| Threshold | Typical Use |
|---:|---|
| `0.00` | Exact pixel match required |
| `0.01` | Default, allows up to 1% changed pixels |
| `0.05` | Looser check for anti-aliasing or dynamic content |

## Development

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo build --release
```

The integration tests cover zero diff, full diff, one-pixel changes, threshold behavior, dimension mismatch, and CLI output.

## Project Structure

```text
src/
  cli.rs          Command-line interface
  diff/           Pixel comparison engine
  report.rs       Verdict and report output
tests/
  integration_test.rs
```

## Project Docs

- [Architecture](docs/ARCHITECTURE.md)
- [Roadmap](docs/ROADMAP.md)
- [Maintainer notes](docs/NOTES.md)
- [Content plan](docs/CONTENT_PLAN.md)

## Release Status

Current production release: `v1.0.0`

The `v1.0.0` release was verified with formatting, clippy, tests, optimized release build, and `cargo package`.

## License

MIT. See [LICENSE](LICENSE).

## Developer

Built by [Sudarshan Chaudhari](https://github.com/SUDARSHANCHAUDHARI) under SudarshanTechLabs.
