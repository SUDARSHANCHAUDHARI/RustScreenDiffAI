use assert_cmd::Command;
use predicates::str::contains;

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
