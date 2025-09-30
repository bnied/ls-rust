//! Sorting functionality for file entries.
//!
//! This module provides different sorting strategies for file listings,
//! including alphabetical and time-based sorting with reverse options.

use crate::file_info::FileInfo;
use std::cmp::Ordering;
use unicase::UniCase;

/// Configuration for sorting behavior
#[allow(clippy::struct_excessive_bools)]
pub struct SortConfig {
    pub by_time: bool,                  // Sort by modification time instead of name
    pub by_size: bool,                  // Sort by file size instead of name
    pub reverse: bool,                  // Reverse the sort order
    pub group_directories_first: bool, // List directories before files
}

impl SortConfig {
    /// Creates a new sort configuration from command-line arguments
    #[allow(clippy::fn_params_excessive_bools)]
    pub fn new(
        by_time: bool,
        by_size: bool,
        reverse: bool,
        group_directories_first: bool,
    ) -> Self {
        SortConfig {
            by_time,
            by_size,
            reverse,
            group_directories_first,
        }
    }
}

/// Sorts entries based on the provided configuration.
/// Supports sorting by name (default), modification time, or file size.
/// Can reverse the sort order and group directories first.
pub fn sort_entries(entries: &mut [FileInfo], config: &SortConfig) {
    entries.sort_by(|a, b| {
        // First, group directories if requested
        if config.group_directories_first {
            let a_is_dir = a.is_dir();
            let b_is_dir = b.is_dir();
            if a_is_dir != b_is_dir {
                return if a_is_dir {
                    Ordering::Less
                } else {
                    Ordering::Greater
                };
            }
        }

        // Then apply the primary sort criterion
        let cmp = if config.by_size {
            // Sort by file size (largest first)
            b.size().cmp(&a.size())
        } else if config.by_time {
            // Sort by modification time (newest first)
            b.modified_time().cmp(&a.modified_time())
        } else {
            // Sort by name (case-insensitive using unicase to avoid allocations)
            UniCase::new(&a.file_name).cmp(&UniCase::new(&b.file_name))
        };

        // Apply reverse if requested
        if config.reverse {
            cmp.reverse()
        } else {
            cmp
        }
    });
}

/// Sorts directories alphabetically for consistent recursive output
pub fn sort_directories(dirs: &mut Vec<&FileInfo>) {
    dirs.sort_by(|a, b| a.file_name.cmp(&b.file_name));
}
