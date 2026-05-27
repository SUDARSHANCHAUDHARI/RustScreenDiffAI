# Notes

## Why This Exists

Visual regressions are easier to discuss when the tool can say exactly how many pixels changed and whether the change crosses a threshold. RustScreenDiffAI keeps that workflow local and simple.

## Known Limits

- It currently compares images pixel by pixel.
- It does not yet support ignored regions or perceptual matching.
- It does not capture screenshots by itself.

## Maintenance Notes

- Keep test fixtures tiny.
- Add expected output examples when CLI behavior changes.
- Be careful with threshold wording because it affects automation trust.
