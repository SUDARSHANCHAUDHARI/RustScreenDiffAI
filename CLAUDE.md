# RustScreenDiffAI — Claude Code Context

## Purpose
Rust CLI that compares two screenshots pixel-by-pixel and reports visual regressions.
Used for signage QA — compare before/after content changes.

## Type
Rust CLI (screendiff)

## Stack
- Language: Rust (stable)
- CLI: clap
- Image processing: image crate
- Serialization: serde + serde_json
- Errors: anyhow + thiserror
- Terminal: colored

## Commands
cargo run -- compare before.png after.png
cargo run -- compare before.png after.png --threshold 0.05
cargo run -- compare before.png after.png --json
cargo test
cargo clippy
cargo fmt
cargo build --release

## GitHub Repo
https://github.com/SUDARSHANCHAUDHARI/RustScreenDiffAI
