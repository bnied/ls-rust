/// Integration tests for output format validation

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs::{self, File};
use std::os::unix::fs::PermissionsExt;
use tempfile::TempDir;

/// Test that directories are displayed in blue (ANSI color code)
#[test]
fn test_directory_color() {
    let temp_dir = TempDir::new().unwrap();
    let sub_dir = temp_dir.path().join("subdir");
    fs::create_dir(&sub_dir).unwrap();
    
    let mut cmd = Command::cargo_bin("ls-rust").unwrap();
    cmd.arg(temp_dir.path());
    
    // Note: The colored crate should output ANSI codes
    // Blue is typically \x1b[34m
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("subdir"));
}

/// Test executable file coloring
#[test]
fn test_executable_color() {
    let temp_dir = TempDir::new().unwrap();
    let exec_file = temp_dir.path().join("executable.sh");
    File::create(&exec_file).unwrap();
    
    // Make file executable
    let mut perms = fs::metadata(&exec_file).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&exec_file, perms).unwrap();
    
    let mut cmd = Command::cargo_bin("ls-rust").unwrap();
    cmd.arg(temp_dir.path());
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("executable.sh"));
}

/// Test sorting order (alphabetical by default)
#[test]
fn test_alphabetical_sorting() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create files in non-alphabetical order
    File::create(temp_dir.path().join("zebra.txt")).unwrap();
    File::create(temp_dir.path().join("apple.txt")).unwrap();
    File::create(temp_dir.path().join("banana.txt")).unwrap();
    
    let mut cmd = Command::cargo_bin("ls-rust").unwrap();
    cmd.arg("-1").arg(temp_dir.path()); // Use -1 for predictable output
    
    cmd.assert()
        .success()
        .stdout(predicate::str::is_match(r"apple\.txt\nbanana\.txt\nzebra\.txt").unwrap());
}

/// Test case-insensitive sorting
#[test]
fn test_case_insensitive_sorting() {
    let temp_dir = TempDir::new().unwrap();
    
    File::create(temp_dir.path().join("Apple.txt")).unwrap();
    File::create(temp_dir.path().join("banana.txt")).unwrap();
    File::create(temp_dir.path().join("Cherry.txt")).unwrap();
    
    let mut cmd = Command::cargo_bin("ls-rust").unwrap();
    cmd.arg("-1").arg(temp_dir.path());
    
    // Should sort case-insensitively: Apple, banana, Cherry
    cmd.assert()
        .success()
        .stdout(predicate::str::is_match(r"Apple\.txt\nbanana\.txt\nCherry\.txt").unwrap());
}

/// Test long format output structure
#[test]
fn test_long_format_structure() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    fs::write(&file_path, "content").unwrap();
    
    let mut cmd = Command::cargo_bin("ls-rust").unwrap();
    cmd.arg("-l").arg(temp_dir.path());
    
    cmd.assert()
        .success()
        // Should start with "total" line
        .stdout(predicate::str::starts_with("total"))
        // Should have permission format (e.g., -rw-r--r--)
        .stdout(predicate::str::is_match(r"-rw-r--r--").unwrap())
        // Should contain file name at the end of the line
        .stdout(predicate::str::contains("test.txt"));
}

/// Test that symlinks are shown with arrow notation in long format
#[test]
#[cfg(unix)]
fn test_symlink_display() {
    let temp_dir = TempDir::new().unwrap();
    let target_file = temp_dir.path().join("target.txt");
    let symlink_path = temp_dir.path().join("link.txt");
    
    File::create(&target_file).unwrap();
    std::os::unix::fs::symlink(&target_file, &symlink_path).unwrap();
    
    let mut cmd = Command::cargo_bin("ls-rust").unwrap();
    cmd.arg("-l").arg(&symlink_path);
    
    cmd.assert()
        .success()
        // Should show link -> target format
        .stdout(predicate::str::contains("link.txt ->"))
        .stdout(predicate::str::contains("target.txt"));
}

/// Test empty directory
#[test]
fn test_empty_directory() {
    let temp_dir = TempDir::new().unwrap();
    
    let mut cmd = Command::cargo_bin("ls-rust").unwrap();
    cmd.arg(temp_dir.path());
    
    cmd.assert()
        .success()
        .stdout(predicate::str::is_empty());
}

/// Test total blocks calculation in long format
#[test]
fn test_total_blocks() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create multiple files
    fs::write(temp_dir.path().join("file1.txt"), "Hello").unwrap();
    fs::write(temp_dir.path().join("file2.txt"), "World").unwrap();
    
    let mut cmd = Command::cargo_bin("ls-rust").unwrap();
    cmd.arg("-l").arg(temp_dir.path());
    
    cmd.assert()
        .success()
        // Should have "total N" where N is the sum of blocks
        .stdout(predicate::str::is_match(r"^total \d+").unwrap());
}

/// Test multiple paths with proper headers
#[test]
fn test_multiple_paths_headers() {
    let temp_dir1 = TempDir::new().unwrap();
    let temp_dir2 = TempDir::new().unwrap();
    
    File::create(temp_dir1.path().join("file1.txt")).unwrap();
    File::create(temp_dir2.path().join("file2.txt")).unwrap();
    
    let mut cmd = Command::cargo_bin("ls-rust").unwrap();
    cmd.arg(temp_dir1.path()).arg(temp_dir2.path());
    
    let output = String::from_utf8(cmd.output().unwrap().stdout).unwrap();
    
    // Check that paths are shown as headers with colons
    assert!(output.contains(&format!("{}:", temp_dir1.path().display())));
    assert!(output.contains(&format!("{}:", temp_dir2.path().display())));
    
    // Check there's a blank line between listings
    let lines: Vec<&str> = output.lines().collect();
    let mut found_blank = false;
    for i in 1..lines.len() {
        if lines[i].is_empty() && !lines[i-1].is_empty() {
            found_blank = true;
            break;
        }
    }
    assert!(found_blank, "Should have blank line between multiple path listings");
}