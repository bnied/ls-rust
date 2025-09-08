//! Utility functions for formatting and display.
//!
//! This module contains helper functions for formatting file permissions,
//! sizes, times, and applying colors to file names based on their type.

use chrono::{DateTime, Local};
use colored::{ColoredString, Colorize};
use std::fs::Metadata;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::time::SystemTime;

/// Converts Unix file permissions to the standard drwxrwxrwx format
pub fn format_permissions(metadata: &Metadata) -> String {
    let mode = metadata.permissions().mode();
    let file_type = if metadata.is_dir() {
        'd'
    } else if metadata.is_symlink() {
        'l'
    } else {
        '-'
    };

    let user = triplet(mode >> 6);
    let group = triplet(mode >> 3);
    let other = triplet(mode);

    format!("{file_type}{user}{group}{other}")
}

/// Converts a 3-bit permission value to rwx format
fn triplet(mode: u32) -> String {
    let r = if mode & 0b100 != 0 { 'r' } else { '-' };
    let w = if mode & 0b010 != 0 { 'w' } else { '-' };
    let x = if mode & 0b001 != 0 { 'x' } else { '-' };
    format!("{r}{w}{x}")
}

/// Formats a system time as a human-readable date string (e.g., "Jan 15 10:30")
pub fn format_time(system_time: SystemTime) -> String {
    let datetime: DateTime<Local> = system_time.into();
    datetime.format("%b %d %H:%M").to_string()
}

/// Converts byte size to human-readable format (B, K, M, G, T, P)
#[allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
pub fn format_size_human(size: u64) -> String {
    const UNITS: &[&str] = &["", "K", "M", "G", "T", "P"];
    let mut size = size as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{:>4}", size as u64)
    } else if size >= 10.0 {
        format!("{:>3.0}{}", size, UNITS[unit_index])
    } else {
        format!("{:>3.1}{}", size, UNITS[unit_index])
    }
}

/// Formats block size for the -s flag, handling platform differences
pub fn format_block_size(metadata: &Metadata) -> String {
    #[cfg(unix)]
    {
        format!("{:>8}", (metadata.blocks() * 512).div_ceil(1024))
    }
    #[cfg(not(unix))]
    {
        // Fallback for non-Unix: show size in KB
        format!("{:>8}", metadata.len().div_ceil(1024))
    }
}

/// Applies color to filename based on file type and permissions
pub fn colorize_name(name: &str, metadata: &Metadata) -> ColoredString {
    let mode = metadata.permissions().mode();

    if metadata.is_dir() {
        // Directories are blue
        name.blue()
    } else if mode & 0o111 != 0 {
        // Executable files (user, group, or other) are red
        name.red()
    } else if mode & 0o004 != 0 {
        // World-readable files are green
        name.green()
    } else {
        // Normal files are white
        name.white()
    }
}
