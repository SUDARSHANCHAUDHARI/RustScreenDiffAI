use anyhow::Result;
use screendiff::report::DiffReport;

pub fn print(report: &DiffReport) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(report)?);
    Ok(())
}
