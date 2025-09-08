/// Integration tests for the ls-rust CLI functionality

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs::{self, File};
use tempfile::TempDir;

/// Test basic directory listing
#[test]
fn test_list_directory() {
    let temp_dir = TempDir::new().unwrap();
    let file1_path = temp_dir.path().join("file1.txt");
    let file2_path = temp_dir.path().join("file2.txt");
    
    File::create(&file1_path).unwrap();
    File::create(&file2_path).unwrap();
    
    let mut cmd = Command::cargo_bin("ls-rust").unwrap();
    cmd.arg(temp_dir.path());
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("file1.txt"))
        .stdout(predicate::str::contains("file2.txt"));
}

/// Test listing with hidden files (-a flag)
#[test]
fn test_list_all_files() {
    let temp_dir = TempDir::new().unwrap();
    let visible_file = temp_dir.path().join("visible.txt");
    let hidden_file = temp_dir.path().join(".hidden");
    
    File::create(&visible_file).unwrap();
    File::create(&hidden_file).unwrap();
    
    // Without -a flag, hidden file should not appear
    let mut cmd = Command::cargo_bin("ls-rust").unwrap();
    cmd.arg(temp_dir.path());
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("visible.txt"))
        .stdout(predicate::str::contains(".hidden").not());
    
    // With -a flag, hidden file should appear
    let mut cmd = Command::cargo_bin("ls-rust").unwrap();
    cmd.arg("-a").arg(temp_dir.path());
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("visible.txt"))
        .stdout(predicate::str::contains(".hidden"));
}

/// Test long format listing (-l flag)
#[test]
fn test_long_format() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    fs::write(&file_path, "Hello, World!").unwrap();
    
    let mut cmd = Command::cargo_bin("ls-rust").unwrap();
    cmd.arg("-l").arg(temp_dir.path());
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test.txt"))
        // Long format should contain permission bits
        .stdout(predicate::str::starts_with("total"))
        .stdout(predicate::str::contains("-rw"));
}

/// Test size display (-s flag)
#[test]
fn test_size_display() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    fs::write(&file_path, "Hello").unwrap();
    
    let mut cmd = Command::cargo_bin("ls-rust").unwrap();
    cmd.arg("-s").arg(temp_dir.path());
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test.txt"))
        // Should show size before filename
        .stdout(predicate::str::is_match(r"\s+\d+\s+test\.txt").unwrap());
}

/// Test single column output (-1 flag)
#[test]
fn test_single_column() {
    let temp_dir = TempDir::new().unwrap();
    let file1_path = temp_dir.path().join("file1.txt");
    let file2_path = temp_dir.path().join("file2.txt");
    
    File::create(&file1_path).unwrap();
    File::create(&file2_path).unwrap();
    
    let mut cmd = Command::cargo_bin("ls-rust").unwrap();
    cmd.arg("-1").arg(temp_dir.path());
    
    cmd.assert()
        .success()
        .stdout(predicate::str::is_match(r"file1\.txt\nfile2\.txt").unwrap());
}

/// Test listing multiple paths
#[test]
fn test_multiple_paths() {
    let temp_dir1 = TempDir::new().unwrap();
    let temp_dir2 = TempDir::new().unwrap();
    
    let file1 = temp_dir1.path().join("file1.txt");
    let file2 = temp_dir2.path().join("file2.txt");
    
    File::create(&file1).unwrap();
    File::create(&file2).unwrap();
    
    let mut cmd = Command::cargo_bin("ls-rust").unwrap();
    cmd.arg(temp_dir1.path()).arg(temp_dir2.path());
    
    // Should show both directory headers and files
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(format!("{}:", temp_dir1.path().display())))
        .stdout(predicate::str::contains(format!("{}:", temp_dir2.path().display())))
        .stdout(predicate::str::contains("file1.txt"))
        .stdout(predicate::str::contains("file2.txt"));
}

/// Test handling of non-existent path
#[test]
fn test_nonexistent_path() {
    let mut cmd = Command::cargo_bin("ls-rust").unwrap();
    cmd.arg("/nonexistent/path/that/should/not/exist");
    
    // Our implementation continues on error, so it returns success
    // but prints error to stderr
    cmd.assert()
        .success()
        .stderr(predicate::str::contains("No such file or directory"));
}

/// Test listing a single file
#[test]
fn test_list_single_file() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("single.txt");
    File::create(&file_path).unwrap();
    
    let mut cmd = Command::cargo_bin("ls-rust").unwrap();
    cmd.arg(&file_path);
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("single.txt"));
}

/// Test human-readable sizes (-h flag with -l or -s)
#[test]
fn test_human_readable_sizes() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    
    // Create a file with some content
    let content = "a".repeat(5000); // 5KB of 'a's
    fs::write(&file_path, content).unwrap();
    
    // Test with -lh
    let mut cmd = Command::cargo_bin("ls-rust").unwrap();
    cmd.arg("-lh").arg(temp_dir.path());
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test.txt"))
        // Should show human-readable size (e.g., "4.9K")
        .stdout(predicate::str::is_match(r"\d+\.\d+K|\d+K").unwrap());
    
    // Test with -sh
    let mut cmd = Command::cargo_bin("ls-rust").unwrap();
    cmd.arg("-sh").arg(temp_dir.path());
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test.txt"))
        .stdout(predicate::str::is_match(r"\d+\.\d+K|\d+K").unwrap());
}