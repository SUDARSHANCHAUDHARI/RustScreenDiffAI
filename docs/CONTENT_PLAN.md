# Content Plan

## Positioning

RustScreenDiffAI is a strong practical testing story: lightweight visual regression checks without starting from a large framework.

## Blog Post Queue

| Priority | Working Title | Feature Tie-In |
| --- | --- | --- |
| 1 | A Simple Rust Screenshot Diff Tool for Visual Regression Checks | Diff image output |
| 2 | Why Pixel Thresholds Matter in Screenshot Testing | Threshold examples |
| 3 | Adding Ignored Regions to Visual Regression Tests | Future ignored regions |

## Auto-Blog Prompt Seed

Write a technical blog post about building a small Rust CLI for screenshot pixel diffs. Include two tiny image fixtures, the compare command, terminal output, JSON output, generated diff image output, and an explanation of threshold-based pass/fail decisions.

## Useful Examples

- `examples/before.ppm`
- `examples/after.ppm`
- `screendiffai compare examples/before.ppm examples/after.ppm --diff-output diff.png`
- JSON output for automation.
