use colored::Colorize;
use crate::report::{DiffReport, Verdict};

pub fn print(report: &DiffReport) {
    println!("\n{}", "ScreenDiff Report".bold().underline());
    println!("{} {}", "Before:".bold(), report.before);
    println!("{} {}", "After:".bold(), report.after);
    println!("{} {}", "Total Pixels:".bold(), report.total_pixels);
    println!("{} {}", "Diff Pixels:".bold(), report.diff_pixels);
    println!("{} {:.2}%", "Diff:".bold(), report.diff_percent * 100.0);
    println!("{} {:.2}%", "Threshold:".bold(), report.threshold * 100.0);

    let verdict_colored = match report.verdict {
        Verdict::Pass => "PASS".green().bold(),
        Verdict::Fail => "FAIL".red().bold(),
    };
    println!("{} {}", "Verdict:".bold(), verdict_colored);
    println!();
}
