# Architecture

RustScreenDiffAI is a CLI image comparison tool for detecting screenshot-level visual differences.

## Goals

- Compare two images with predictable pixel-diff behavior.
- Report changed pixels, percentage, threshold, and verdict.
- Support terminal and JSON output.
- Keep the diff engine independent from CLI rendering.

## Module Layout

| Module | Responsibility |
| --- | --- |
| `src/cli.rs` | CLI command and threshold parsing |
| `src/diff/` | Image loading and pixel comparison |
| `src/report.rs` | Diff report model |
| `src/output/` | Terminal and JSON rendering |

## Data Flow

1. The CLI receives before and after image paths.
2. The diff module loads both images and validates dimensions.
3. Pixel comparison counts changed pixels.
4. The report computes percentage and pass/fail verdict.
5. The renderer prints terminal or JSON output.

## Design Notes

- Dimension mismatches are explicit errors.
- Threshold semantics should stay stable.
- Fixtures should stay small and easy to inspect.
- Future features should preserve deterministic output.

## Release Assumptions

- `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`, and `cargo package` pass before release.
- GitHub Actions are intentionally not used in this repo.
