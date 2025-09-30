//! Formatting module for displaying file information.
//!
//! This module provides a flexible formatter that can display files in different formats
//! using the Display trait, making it easy to test and extend.

use crate::file_info::FileInfo;
use crate::utils::{
    colorize_name, format_block_size, format_permissions, format_size_human, format_time,
};
use std::sync::LazyLock;
use std::collections::HashMap;
use std::fmt;
use std::sync::Mutex;
use std::time::SystemTime;
use users::{get_group_by_gid, get_user_by_uid};

/// Cache for user ID to username lookups
static USER_CACHE: LazyLock<Mutex<HashMap<u32, String>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Cache for group ID to group name lookups
static GROUP_CACHE: LazyLock<Mutex<HashMap<u32, String>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Gets username for a UID, using cache to avoid repeated lookups
fn get_username(uid: u32) -> String {
    let mut cache = USER_CACHE.lock().unwrap();
    cache
        .entry(uid)
        .or_insert_with(|| {
            get_user_by_uid(uid).map_or_else(
                || uid.to_string(),
                |u| u.name().to_string_lossy().to_string(),
            )
        })
        .clone()
}

/// Gets group name for a GID, using cache to avoid repeated lookups
fn get_groupname(gid: u32) -> String {
    let mut cache = GROUP_CACHE.lock().unwrap();
    cache
        .entry(gid)
        .or_insert_with(|| {
            get_group_by_gid(gid).map_or_else(
                || gid.to_string(),
                |g| g.name().to_string_lossy().to_string(),
            )
        })
        .clone()
}

/// Display format for file entries
#[derive(Debug, PartialEq, Clone)]
pub enum Format {
    Name,     // Display only the file name (colored)
    WithSize, // Display size followed by file name
    Long,     // Display full details (permissions, owner, size, date, name)
}

/// Formatter for displaying `FileInfo` in various formats.
/// Implements Display trait for easy rendering and testing.
pub struct FileInfoFormatter<'a> {
    pub file_info: &'a FileInfo, // Reference to the file information to display
    pub format: Format,          // The format to use for display
    pub human_readable: bool,    // Whether to use human-readable sizes (K, M, G)
}

impl fmt::Display for FileInfoFormatter<'_> {
    /// Formats the `FileInfo` according to the selected format.
    /// This allows using the formatter with println!, format!, etc.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.format {
            Format::Name => {
                let colored_name =
                    colorize_name(&self.file_info.file_name, &self.file_info.metadata);
                write!(f, "{colored_name}")
            }
            Format::WithSize => {
                let size = if self.human_readable {
                    format_size_human(self.file_info.size())
                } else {
                    format_block_size(&self.file_info.metadata)
                };
                let colored_name =
                    colorize_name(&self.file_info.file_name, &self.file_info.metadata);
                write!(f, "{size} {colored_name}")
            }
            Format::Long => {
                write!(f, "{}", self.format_long())
            }
        }
    }
}

impl FileInfoFormatter<'_> {
    /// Formats file information in long format (similar to ls -l).
    /// Includes permissions, links, owner, group, size, date, and name.
    /// For symlinks, also shows the target path.
    fn format_long(&self) -> String {
        let permissions = format_permissions(&self.file_info.metadata);
        let nlink = self.file_info.nlink();
        let owner = get_username(self.file_info.uid());
        let group = get_groupname(self.file_info.gid());
        let size = if self.human_readable {
            format_size_human(self.file_info.size())
        } else {
            self.file_info.size().to_string()
        };
        let modified = format_time(self.file_info.modified().unwrap_or(SystemTime::UNIX_EPOCH));

        let mut display_name = colorize_name(&self.file_info.file_name, &self.file_info.metadata);

        // If it's a symlink, show the target (or error if broken)
        if let Some(target_result) = self.file_info.symlink_target() {
            match target_result {
                Ok(target) => {
                    display_name = format!("{} -> {}", display_name, target.display()).into();
                }
                Err(err) => {
                    display_name = format!("{display_name} -> [{err}]").into();
                }
            }
        }

        format!("{permissions} {nlink:>3} {owner} {group} {size:>8} {modified} {display_name}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use tempfile::TempDir;

    #[test]
    fn test_format_name() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        File::create(&file_path).unwrap();

        let file_info = FileInfo::from_path(&file_path).unwrap();
        let formatter = FileInfoFormatter {
            file_info: &file_info,
            format: Format::Name,
            human_readable: false,
        };

        let output = format!("{}", formatter);
        assert!(output.contains("test.txt"));
    }

    #[test]
    fn test_format_with_size() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        fs::write(&file_path, "Hello, World!").unwrap();

        let file_info = FileInfo::from_path(&file_path).unwrap();
        let formatter = FileInfoFormatter {
            file_info: &file_info,
            format: Format::WithSize,
            human_readable: true,
        };

        let output = format!("{}", formatter);
        assert!(output.contains("test.txt"));
        // Should contain size information
        assert!(output.len() > "test.txt".len());
    }

    #[test]
    fn test_format_long() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        File::create(&file_path).unwrap();

        let file_info = FileInfo::from_path(&file_path).unwrap();
        let formatter = FileInfoFormatter {
            file_info: &file_info,
            format: Format::Long,
            human_readable: false,
        };

        let output = format!("{}", formatter);
        // Long format should contain permissions
        assert!(output.starts_with("-") || output.starts_with("d") || output.starts_with("l"));
        assert!(output.contains("test.txt"));
    }
}
