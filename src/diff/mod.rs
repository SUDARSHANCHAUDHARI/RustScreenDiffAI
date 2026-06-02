use anyhow::{bail, Result};
use image::{GenericImageView, Rgb};

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

pub fn write_diff_image(before_path: &str, after_path: &str, output_path: &str) -> Result<()> {
    let before = image::open(before_path)?;
    let after = image::open(after_path)?;

    if before.dimensions() != after.dimensions() {
        bail!(
            "Image dimensions differ: {:?} vs {:?}",
            before.dimensions(),
            after.dimensions()
        );
    }

    let before_rgb = before.to_rgb8();
    let after_rgb = after.to_rgb8();
    let mut diff = before_rgb.clone();

    for (x, y, pixel) in diff.enumerate_pixels_mut() {
        if before_rgb.get_pixel(x, y) != after_rgb.get_pixel(x, y) {
            *pixel = Rgb([255, 0, 255]);
        }
    }

    diff.save(output_path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageBuffer, Rgb};
    use tempfile::NamedTempFile;

    fn save_solid_image(color: Rgb<u8>, w: u32, h: u32) -> NamedTempFile {
        let f = NamedTempFile::with_suffix(".png").unwrap();
        let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_fn(w, h, |_, _| color);
        img.save(f.path()).unwrap();
        f
    }

    #[test]
    fn identical_images_have_zero_diff() {
        let a = save_solid_image(Rgb([100, 150, 200]), 10, 10);
        let b = save_solid_image(Rgb([100, 150, 200]), 10, 10);
        let result = compare(a.path().to_str().unwrap(), b.path().to_str().unwrap(), 0.01).unwrap();
        assert_eq!(result.diff_pixels, 0);
        assert_eq!(result.diff_percent, 0.0);
    }

    #[test]
    fn fully_different_images_have_100_percent_diff() {
        let a = save_solid_image(Rgb([0, 0, 0]), 4, 4);
        let b = save_solid_image(Rgb([255, 255, 255]), 4, 4);
        let result = compare(a.path().to_str().unwrap(), b.path().to_str().unwrap(), 0.5).unwrap();
        assert_eq!(result.diff_pixels, 16);
        assert!((result.diff_percent - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn mismatched_dimensions_returns_error() {
        let a = save_solid_image(Rgb([0, 0, 0]), 4, 4);
        let b = save_solid_image(Rgb([0, 0, 0]), 8, 8);
        assert!(compare(a.path().to_str().unwrap(), b.path().to_str().unwrap(), 0.0).is_err());
    }

    #[test]
    fn write_diff_image_creates_output_file() {
        let a = save_solid_image(Rgb([0, 0, 0]), 4, 4);
        let b = save_solid_image(Rgb([255, 0, 0]), 4, 4);
        let out = NamedTempFile::with_suffix(".png").unwrap();
        write_diff_image(
            a.path().to_str().unwrap(),
            b.path().to_str().unwrap(),
            out.path().to_str().unwrap(),
        )
        .unwrap();
        assert!(out.path().exists());
    }
}
