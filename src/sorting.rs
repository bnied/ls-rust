//! Sorting functionality for file entries.
//!
//! This module provides different sorting strategies for file listings,
//! including alphabetical and time-based sorting with reverse options.

use crate::file_info::FileInfo;
use std::cmp::Ordering;

/// Configuration for sorting behavior
pub struct SortConfig {
    pub by_time: bool, // Sort by modification time instead of name
    pub reverse: bool, // Reverse the sort order
}

impl SortConfig {
    /// Creates a new sort configuration from command-line arguments
    pub fn new(by_time: bool, reverse: bool) -> Self {
        SortConfig { by_time, reverse }
    }
}

/// Sorts entries based on the provided configuration.
/// Supports sorting by name (default) or modification time.
/// Can reverse the sort order.
pub fn sort_entries(entries: &mut [FileInfo], config: &SortConfig) {
    entries.sort_by(|a, b| {
        let cmp = if config.by_time {
            // Sort by modification time (newest first)
            b.modified_time().cmp(&a.modified_time())
        } else {
            // Sort by name (case-insensitive)
            a.file_name.to_lowercase().cmp(&b.file_name.to_lowercase())
        };

        // Apply reverse if requested
        if config.reverse {
            match cmp {
                Ordering::Less => Ordering::Greater,
                Ordering::Greater => Ordering::Less,
                Ordering::Equal => Ordering::Equal,
            }
        } else {
            cmp
        }
    });
}

/// Sorts directories alphabetically for consistent recursive output
pub fn sort_directories(dirs: &mut Vec<&FileInfo>) {
    dirs.sort_by(|a, b| a.file_name.cmp(&b.file_name));
}
