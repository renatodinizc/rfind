use assert_cmd::Command;
use std::process::Output;
use std::str::from_utf8;

#[test]
fn test_program() {
    let mut cmd = Command::cargo_bin("rfind").unwrap();

    cmd.arg("tests").arg("-t").arg("l");

    cmd.assert()
        .success()
        .stdout("tests/dir2/subdir/symlink1.txt\n");
}

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
    let expected_files = [
        "tests/dir1/file1.txt",
        "tests/dir2/file2.txt",
        "tests/dir2/subdir/file3.txt",
    ];
    let unexpected_file = "anotherfile.txt";

    let output = run_rfind_with_args(&args);
    assert!(output.status.success(), "Command did not run successfully.");
    let stdout = from_utf8(&output.stdout).expect("Output not valid UTF-8");

    for file in expected_files.iter() {
        assert!(
            stdout.contains(file),
            "Expected file {} not found in output",
            file
        );
    }

    assert!(
        !stdout.contains(unexpected_file),
        "Unexpected file {} found in output",
        unexpected_file
    );
}

#[test]
fn test_find_directories_only() {
    let args = ["tests", "-t", "d"];
    let expected_directories = ["tests/dir1", "tests/dir2", "tests/dir2/subdir"];
    let unexpected_file = "tests/dir1/file1.txt";

    let output = run_rfind_with_args(&args);
    assert!(output.status.success(), "Command did not run successfully.");
    let stdout = from_utf8(&output.stdout).expect("Output not valid UTF-8");

    for dir in expected_directories.iter() {
        assert!(
            stdout.contains(dir),
            "Expected directory {} not found in output",
            dir
        );
    }
    assert!(
        !stdout.contains(unexpected_file),
        "Unexpected file {} found in output",
        unexpected_file
    );
}

#[test]
fn test_find_files_with_name_pattern() {
    let args = ["tests", "-t", "f", "-n", "file[1-3]\\.txt"];
    let expected_files = [
        "tests/dir1/file1.txt",
        "tests/dir2/file2.txt",
        "tests/dir2/subdir/file3.txt",
    ];
    let unexpected_file = "tests/cli.rs";

    let output = run_rfind_with_args(&args);
    assert!(output.status.success(), "Command did not run successfully.");
    let stdout = from_utf8(&output.stdout).expect("Output not valid UTF-8");

    for file in expected_files.iter() {
        assert!(
            stdout.contains(file),
            "Expected file {} not found in output",
            file
        );
    }
    assert!(
        !stdout.contains(unexpected_file),
        "Unexpected file {} found in output",
        unexpected_file
    );
}

#[test]
fn test_find_symlinks_only() {
    let args = ["tests", "-t", "l"];
    let expected_symlinks = ["tests/dir2/subdir/symlink1.txt"];
    let unexpected_file = "tests/dir1/file1.txt";

    let output = run_rfind_with_args(&args);
    assert!(output.status.success(), "Command did not run successfully.");
    let stdout = from_utf8(&output.stdout).expect("Output not valid UTF-8");

    for symlink in expected_symlinks.iter() {
        assert!(
            stdout.contains(symlink),
            "Expected symlink {} not found in output",
            symlink
        );
    }
    assert!(
        !stdout.contains(unexpected_file),
        "Unexpected file {} found in output",
        unexpected_file
    );
}

#[test]
fn test_find_csv_files_only() {
    let args = ["tests", "-n", "\\.csv$"];
    let expected_csv_files = [
        "tests/dir2/file.csv",
    ];
    let unexpected_file = "tests/dir1/file1.txt";

    let output = run_rfind_with_args(&args);
    assert!(output.status.success(), "Command did not run successfully.");
    let stdout = from_utf8(&output.stdout).expect("Output not valid UTF-8");

    for file in expected_csv_files.iter() {
        assert!(
            stdout.contains(file),
            "Expected CSV file {} not found in output",
            file
        );
    }
    assert!(
        !stdout.contains(unexpected_file),
        "Unexpected file {} found in output",
        unexpected_file
    );
}

#[test]
fn test_find_files_in_specific_directory() {
    let args = ["tests/dir2"];
    let expected_files_in_dir2 = [
        "tests/dir2/file2.txt",
        "tests/dir2/file.csv",
        "tests/dir2/subdir/file3.txt",
    ];
    let unexpected_file = "tests/dir1/file1.txt";

    let output = run_rfind_with_args(&args);
    assert!(output.status.success(), "Command did not run successfully.");
    let stdout = from_utf8(&output.stdout).expect("Output not valid UTF-8");

    for file in expected_files_in_dir2.iter() {
        assert!(
            stdout.contains(file),
            "Expected file {} not found in output within dir2",
            file
        );
    }
    assert!(
        !stdout.contains(unexpected_file),
        "File {} found in output but was outside dir2",
        unexpected_file
    );
}
