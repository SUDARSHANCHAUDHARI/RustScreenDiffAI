use assert_cmd::Command;
use image::{ImageBuffer, Rgb};
use predicates::str::contains;
use screendiff::diff;
use screendiff::report::{self, Verdict};
use tempfile::NamedTempFile;

// Helper: write a solid-colour PNG to a temp file
fn solid_png(r: u8, g: u8, b: u8) -> NamedTempFile {
    let img: ImageBuffer<Rgb<u8>, _> = ImageBuffer::from_fn(100, 100, |_, _| Rgb([r, g, b]));
    let file = tempfile::Builder::new().suffix(".png").tempfile().unwrap();
    img.save(file.path()).unwrap();
    file
}

// Helper: 100x100 red PNG with one pixel changed to green
fn mostly_red_png() -> NamedTempFile {
    let mut img: ImageBuffer<Rgb<u8>, _> =
        ImageBuffer::from_fn(100, 100, |_, _| Rgb([255u8, 0, 0]));
    img.put_pixel(0, 0, Rgb([0, 255, 0]));
    let file = tempfile::Builder::new().suffix(".png").tempfile().unwrap();
    img.save(file.path()).unwrap();
    file
}

// --- CLI tests ---

#[test]
fn test_help() {
    Command::cargo_bin("screendiff")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(contains("pixel diff"));
}

#[test]
fn test_compare_help() {
    Command::cargo_bin("screendiff")
        .unwrap()
        .args(["compare", "--help"])
        .assert()
        .success()
        .stdout(contains("before screenshot"));
}

// --- Diff engine unit tests ---

#[test]
fn test_identical_images_zero_diff() {
    let a = solid_png(255, 0, 0);
    let b = solid_png(255, 0, 0);
    let result = diff::compare(
        a.path().to_str().unwrap(),
        b.path().to_str().unwrap(),
        0.01,
    )
    .unwrap();
    assert_eq!(result.diff_pixels, 0);
    assert_eq!(result.diff_percent, 0.0);
}

#[test]
fn test_completely_different_images_full_diff() {
    let a = solid_png(255, 0, 0);
    let b = solid_png(0, 0, 255);
    let result = diff::compare(
        a.path().to_str().unwrap(),
        b.path().to_str().unwrap(),
        0.01,
    )
    .unwrap();
    assert_eq!(result.diff_pixels, 10_000);
    assert_eq!(result.diff_percent, 1.0);
}

#[test]
fn test_one_pixel_diff_counted() {
    let a = solid_png(255, 0, 0);
    let b = mostly_red_png();
    let result = diff::compare(
        a.path().to_str().unwrap(),
        b.path().to_str().unwrap(),
        0.01,
    )
    .unwrap();
    assert_eq!(result.diff_pixels, 1);
}

#[test]
fn test_dimension_mismatch_returns_error() {
    let a = solid_png(255, 0, 0);
    let small_img: ImageBuffer<Rgb<u8>, _> =
        ImageBuffer::from_fn(50, 50, |_, _| Rgb([255u8, 0, 0]));
    let small = tempfile::Builder::new().suffix(".png").tempfile().unwrap();
    small_img.save(small.path()).unwrap();

    let result = diff::compare(
        a.path().to_str().unwrap(),
        small.path().to_str().unwrap(),
        0.01,
    );
    assert!(result.is_err());
}

// --- Report / verdict tests ---

#[test]
fn test_zero_diff_verdict_is_pass() {
    let a = solid_png(0, 255, 0);
    let b = solid_png(0, 255, 0);
    let result = diff::compare(
        a.path().to_str().unwrap(),
        b.path().to_str().unwrap(),
        0.01,
    )
    .unwrap();
    let rep = report::build("a.png", "b.png", &result);
    assert!(matches!(rep.verdict, Verdict::Pass));
}

#[test]
fn test_over_threshold_verdict_is_fail() {
    let a = solid_png(255, 0, 0);
    let b = solid_png(0, 0, 255);
    let result = diff::compare(
        a.path().to_str().unwrap(),
        b.path().to_str().unwrap(),
        0.01,
    )
    .unwrap();
    let rep = report::build("a.png", "b.png", &result);
    assert!(matches!(rep.verdict, Verdict::Fail));
}

#[test]
fn test_single_pixel_diff_within_threshold_passes() {
    // 1 changed pixel in 10000 = 0.0001 — well under 1% threshold
    let a = solid_png(255, 0, 0);
    let b = mostly_red_png();
    let result = diff::compare(
        a.path().to_str().unwrap(),
        b.path().to_str().unwrap(),
        0.01,
    )
    .unwrap();
    let rep = report::build("a.png", "b.png", &result);
    assert!(matches!(rep.verdict, Verdict::Pass));
}

// --- CLI + real image files ---

#[test]
fn test_cli_identical_images_json_pass() {
    let a = solid_png(128, 128, 128);
    let b = solid_png(128, 128, 128);
    Command::cargo_bin("screendiff")
        .unwrap()
        .args([
            "compare",
            a.path().to_str().unwrap(),
            b.path().to_str().unwrap(),
            "--json",
        ])
        .assert()
        .success()
        .stdout(contains("Pass"));
}

#[test]
fn test_cli_different_images_json_fail() {
    let a = solid_png(255, 0, 0);
    let b = solid_png(0, 0, 255);
    Command::cargo_bin("screendiff")
        .unwrap()
        .args([
            "compare",
            a.path().to_str().unwrap(),
            b.path().to_str().unwrap(),
            "--json",
        ])
        .assert()
        .success()
        .stdout(contains("Fail"));
}
