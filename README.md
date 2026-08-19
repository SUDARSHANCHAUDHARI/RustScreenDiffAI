# RustScreenDiffAI

[![crates.io](https://img.shields.io/crates/v/screendiffai?logo=rust)](https://crates.io/crates/screendiffai)
[![Downloads](https://img.shields.io/crates/d/screendiffai?logo=rust)](https://crates.io/crates/screendiffai)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange?logo=rust)

> A Rust CLI pixel-diff engine for screenshot regression checks — deterministic `PASS`/`FAIL` verdicts.

**RustScreenDiffAI** (installed as the `screendiffai` command) compares two images pixel
by pixel, calculates the changed-pixel percentage, and returns a clear `PASS` or `FAIL`
verdict against a configurable threshold.

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Included Example](#included-example)
- [How It Works](#how-it-works)
- [Example Output](#example-output)
- [Threshold Guidance](#threshold-guidance)
- [Development](#development)
- [Project Structure](#project-structure)
- [Documentation](#documentation)
- [Release Status](#release-status)
- [License](#license)
- [About](#about)

## Overview

Visual regressions are easiest to miss when screens look "mostly fine." RustScreenDiffAI
provides a deterministic, script-friendly image comparison step that can be used in QA
workflows, CI jobs, screenshot tests, and digital signage review pipelines.

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

### From crates.io (recommended)

```bash
cargo install screendiffai
```

### From source

```bash
git clone https://github.com/SUDARSHANCHAUDHARI/RustScreenDiffAI.git
cd RustScreenDiffAI
cargo build --release
```

The binary is created at:

```bash
target/release/screendiffai
```

Optional local install from a source checkout:

```bash
cargo install --path .
```

## Usage

```bash
# Compare two screenshots with the default 1% threshold
screendiffai compare before.png after.png

# Allow up to 5% difference
screendiffai compare before.png after.png --threshold 0.05

# Emit JSON
screendiffai compare before.png after.png --json

# Write a visual diff image with changed pixels highlighted in magenta
screendiffai compare before.png after.png --diff-output diff.png
```

## Included Example

The repository includes two tiny PPM fixtures:

```bash
screendiffai compare examples/before.ppm examples/after.ppm --threshold 0.10

screendiffai compare examples/before.ppm examples/after.ppm --diff-output diff.png
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
6. Optionally write a diff image where changed pixels are magenta and unchanged pixels come from the baseline image.
7. Return `PASS` when `diff_percent <= threshold`; otherwise return `FAIL`.

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

The integration tests cover zero diff, full diff, one-pixel changes, threshold behavior,
dimension mismatch, CLI output, and visual diff image generation.

## Project Structure

```text
src/
  cli.rs          Command-line interface
  diff/           Pixel comparison engine
  report.rs       Verdict and report output
tests/
  integration_test.rs
```

## Documentation

- [Architecture](docs/ARCHITECTURE.md)
- [Roadmap](docs/ROADMAP.md)
- [Maintainer notes](docs/NOTES.md)
- [Content plan](docs/CONTENT_PLAN.md)

## Release Status

Current release: **`v1.1.1`**, published on [crates.io](https://crates.io/crates/screendiffai).

Each release is verified with formatting, Clippy, tests, an optimized release build, and
`cargo package` before publishing.

## License

MIT — see [LICENSE](LICENSE).

---

## About

I'm Sudarshan Chaudhari, a Senior Quality Engineer, Test Automation specialist, and AI systems builder based in Bangkok, Thailand.

I have 13+ years of experience in software quality engineering, working across SaaS, fintech, gaming, web, mobile, cloud, and digital signage platforms. My background combines hands-on test automation with QA leadership, test strategy, CI/CD, release quality, production investigation, and cross-platform validation.

Alongside my professional QA career, I run [SudarshanTechLabs](https://sudarshantechlabs.com/), my independent engineering and product lab where I design, build, test, and ship software across Android, web, AI, cybersecurity, developer tooling, and cross-platform applications.

### What I work on

- ⚙️ **Quality Engineering & Test Automation** — Playwright, Selenium, Cypress, Appium, API testing, automation frameworks, end-to-end testing, CI/CD, release gates, GitHub Actions, risk-based testing, and production validation
- 🤖 **AI Systems & Automation** — AI agents, multi-agent orchestration, MCP servers, AI-assisted QA, prompt tooling, developer workflows, automation systems, and Claude Code plugins
- 📱 **Mobile & Cross-Platform Applications** — Android applications built with Kotlin and Jetpack Compose, Google Play releases, automated build and publishing pipelines, and cross-platform development spanning iOS, web, Windows, and macOS
- 🌐 **Web Applications & Platforms** — Full-stack applications using Next.js, TypeScript, Firebase, Cloudflare, REST APIs, and modern web infrastructure
- 🛠️ **Developer Tooling & CLI Engineering** — Rust, Python, TypeScript, CLI utilities, multi-repository tooling, build automation, release tooling, and engineering productivity systems
- 🛡️ **Cybersecurity & Observability** — Threat detection, log analysis, security auditing, vulnerability assessment, monitoring, and security-focused developer tools
- 📺 **Digital Signage & Device Platforms** — Content validation, playback testing, device compatibility, production investigation, monitoring, and QA across diverse hardware and operating-system environments

My work sits at the intersection of quality engineering, automation, AI, and software development. I approach products with a QA mindset from the beginning: understanding failure modes, designing for testability, automating repetitive work, and building release confidence into the engineering process.

Through SudarshanTechLabs, I also build products and tools from idea to production, covering architecture, development, testing, CI/CD, release automation, monitoring, and ongoing maintenance.

🌐 [sudarshantechlabs.com](https://sudarshantechlabs.com/) · 💼 [LinkedIn](https://linkedin.com/in/sudarshan-chaudhari) · 🐙 [GitHub](https://github.com/SUDARSHANCHAUDHARI) · ✉️ [sunny.sudarshan@gmail.com](mailto:sunny.sudarshan@gmail.com)
