use assert_cmd::Command;
use std::process::Output;
use std::str::from_utf8;

fn run_rfind_with_args(args: &[&str]) -> Output {
    Command::cargo_bin("rfind")
        .unwrap()
        .args(args)
        .output()
        .expect("Failed to execute command")
}

#[test]
fn test_find_files_only() {
    let args = ["tests", "-t", "f"];

    let output = run_rfind_with_args(&args);
    let stdout = from_utf8(&output.stdout).expect("Output not valid UTF-8");

    assert!(output.status.success());
    assert!(stdout.contains("tests/dir1/file1.txt"));
    assert!(stdout.contains("tests/dir2/file2.txt"));
    assert!(stdout.contains("tests/dir2/subdir/file3.txt"));
}

#[test]
fn test_find_directories_only() {
    let args = ["tests", "-t", "d"];

    let output = run_rfind_with_args(&args);
    let stdout = from_utf8(&output.stdout).expect("Output not valid UTF-8");

    assert!(output.status.success());
    assert!(!stdout.contains("tests/dir1/file1.txt"));
    assert!(stdout.contains("tests/dir1"));
    assert!(stdout.contains("tests/dir2"));
    assert!(stdout.contains("tests/dir2/subdir"));
}

#[test]
fn test_find_files_with_name_pattern() {
    let args = ["tests", "-t", "f", "-n", "file?.txt"];

    let output = run_rfind_with_args(&args);
    let stdout = from_utf8(&output.stdout).expect("Output not valid UTF-8");

    assert!(output.status.success());
    assert!(!stdout.contains("tests/cli.rs"));
    assert!(stdout.contains("tests/dir1/file1.txt"));
    assert!(stdout.contains("tests/dir2/file2.txt"));
    assert!(stdout.contains("tests/dir2/subdir/file3.txt"));
}

#[test]
fn test_find_symlinks_only() {
    let args = ["tests", "-t", "l"];

    let output = run_rfind_with_args(&args);
    let stdout = from_utf8(&output.stdout).expect("Output not valid UTF-8");

    assert!(output.status.success());
    assert!(!stdout.contains("tests/dir1/file1.txt"));
    assert!(stdout.contains("tests/dir2/subdir/symlink1.txt"));
}

#[test]
fn test_find_csv_files_only() {
    let args = ["tests", "-n", "*.csv"];

    let output = run_rfind_with_args(&args);
    let stdout = from_utf8(&output.stdout).expect("Output not valid UTF-8");

    assert!(output.status.success());
    assert!(!stdout.contains("tests/dir1/file1.txt"));
    assert!(stdout.contains("tests/dir2/file.csv"));
}

#[test]
fn test_find_files_in_specific_directory() {
    let args = ["tests/dir2"];

    let output = run_rfind_with_args(&args);
    let stdout = from_utf8(&output.stdout).expect("Output not valid UTF-8");

    assert!(output.status.success());
    assert!(!stdout.contains("tests/dir1/file1.txt"));
    assert!(stdout.contains("tests/dir2/file.csv"));
    assert!(stdout.contains("tests/dir2/file2.txt"));
    assert!(stdout.contains("tests/dir2/subdir/file3.txt"));
}

#[test]
#[should_panic]
fn test_find_files_with_complex_regex_pattern_should_fail() {
    let args = ["tests", "-n", "file[12]\\.txt"];

    let output = run_rfind_with_args(&args);
    let stdout = from_utf8(&output.stdout).expect("Output not valid UTF-8");

    assert!(output.status.success());
    assert!(stdout.contains("tests/dir1/file1.txt"));
    assert!(stdout.contains("tests/dir2/file2.txt"));
    assert!(!stdout.contains("tests/dir2/subdir/file3.txt"));
    assert!(!stdout.contains("tests/cli.rs"));
}
