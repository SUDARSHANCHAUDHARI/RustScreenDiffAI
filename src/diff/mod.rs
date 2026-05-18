use anyhow::{bail, Result};
use image::GenericImageView;

pub struct DiffResult {
    pub total_pixels: u64,
    pub diff_pixels: u64,
    pub diff_percent: f64,
    pub threshold: f64,
}

pub fn compare(before_path: &str, after_path: &str, threshold: f64) -> Result<DiffResult> {
    let before = image::open(before_path)?;
    let after = image::open(after_path)?;

    if before.dimensions() != after.dimensions() {
        bail!(
            "Image dimensions differ: {:?} vs {:?}",
            before.dimensions(),
            after.dimensions()
        );
    }

    let (width, height) = before.dimensions();
    let total_pixels = (width * height) as u64;
    let before_rgb = before.to_rgb8();
    let after_rgb = after.to_rgb8();

    let diff_pixels = before_rgb
        .pixels()
        .zip(after_rgb.pixels())
        .filter(|(a, b)| a != b)
        .count() as u64;

    let diff_percent = diff_pixels as f64 / total_pixels as f64;

    Ok(DiffResult {
        total_pixels,
        diff_pixels,
        diff_percent,
        threshold,
    })
}
