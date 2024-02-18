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

#[test]
fn test_max_depth_limit() {
    let args = ["tests", "--max-depth", "1"];

    let output = run_rfind_with_args(&args);
    let stdout = from_utf8(&output.stdout).expect("Output not valid UTF-8");

    assert!(output.status.success());
    assert!(stdout.contains("tests/dir1"));
    assert!(stdout.contains("tests/dir2"));
    assert!(!stdout.contains("tests/dir1/file1.txt"));
    assert!(!stdout.contains("tests/dir2/file2.txt"));
}

#[test]
fn test_min_depth_limit() {
    let args = ["tests", "--min-depth", "3"];

    let output = run_rfind_with_args(&args);
    let stdout = from_utf8(&output.stdout).expect("Output not valid UTF-8");

    assert!(output.status.success());
    assert!(stdout.contains("tests/dir2/subdir/symlink1.txt"));
    assert!(stdout.contains("tests/dir2/subdir/file3.txt"));
    assert!(!stdout.contains("tests/dir1"));
    assert!(!stdout.contains("tests/cli.rs"));
}

#[test]
fn test_find_files_larger_than_size() {
    let args = ["tests", "--size", "4"]; // Find files larger than 1 KiB

    let output = run_rfind_with_args(&args);
    let stdout = from_utf8(&output.stdout).expect("Output not valid UTF-8");

    assert!(output.status.success());
    assert!(stdout.contains("tests/cli.rs"));
    assert!(!stdout.contains("tests/dir1/file1.txt"));
}

#[test]
fn test_find_files_smaller_than_size() {
    let args = ["tests", "--size", "-1"]; // Find files smaller than 1 KiB

    let output = run_rfind_with_args(&args);
    let stdout = from_utf8(&output.stdout).expect("Output not valid UTF-8");

    assert!(output.status.success());
    assert!(!stdout.contains("tests/cli.rs"));
    assert!(stdout.contains("tests/dir2/file2.txt"));
    assert!(stdout.contains("tests/dir2/subdir/symlink1.txt"));
    assert!(stdout.contains("tests/dir2/subdir/file3.txt"));
    assert!(stdout.contains("tests/dir2/file.csv"));
    assert!(stdout.contains("tests/dir1/file1.txt"));
}
