//! Directory reading and traversal functionality.
//!
//! This module handles reading directory contents, filtering entries,
//! and managing recursive directory traversal.

use crate::file_info::FileInfo;
use std::fs;
use std::io;
use std::path::Path;

/// Reads a directory and collects file information.
/// Filters hidden files based on the show_all flag.
/// Continues processing even if some entries fail to read.
pub fn collect_entries(dir: &Path, show_all: bool) -> io::Result<Vec<FileInfo>> {
    let mut entries = vec![];
    let mut errors = vec![];

    for entry in fs::read_dir(dir)? {
        match entry {
            Ok(entry) => {
                let file_name = entry.file_name().to_string_lossy().to_string();

                // Skip hidden files unless -a flag is set
                if !show_all && file_name.starts_with('.') {
                    continue;
                }

                match FileInfo::from_entry(entry) {
                    Ok(file_info) => entries.push(file_info),
                    Err(e) => errors.push(e),
                }
            }
            Err(e) => errors.push(e),
        }
    }

    // Report errors but continue processing
    for error in errors {
        eprintln!("ls: {error}");
    }

    Ok(entries)
}

/// Filters directory entries for recursive traversal.
/// Returns only non-hidden directories from the given entries.
pub fn get_subdirectories(entries: &[FileInfo]) -> Vec<&FileInfo> {
    entries
        .iter()
        .filter(|f| f.is_dir() && !f.is_hidden())
        .collect()
}
