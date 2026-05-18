use serde::{Deserialize, Serialize};
use crate::diff::DiffResult;

#[derive(Debug, Serialize, Deserialize)]
pub struct DiffReport {
    pub before: String,
    pub after: String,
    pub total_pixels: u64,
    pub diff_pixels: u64,
    pub diff_percent: f64,
    pub verdict: Verdict,
    pub threshold: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Verdict {
    Pass,
    Fail,
}

impl std::fmt::Display for Verdict {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Verdict::Pass => write!(f, "PASS"),
            Verdict::Fail => write!(f, "FAIL"),
        }
    }
}

pub fn build(before: &str, after: &str, result: &DiffResult) -> DiffReport {
    let verdict = if result.diff_percent <= result.threshold {
        Verdict::Pass
    } else {
        Verdict::Fail
    };
    DiffReport {
        before: before.to_string(),
        after: after.to_string(),
        total_pixels: result.total_pixels,
        diff_pixels: result.diff_pixels,
        diff_percent: result.diff_percent,
        verdict,
        threshold: result.threshold,
    }
}
