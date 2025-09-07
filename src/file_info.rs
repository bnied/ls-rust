//! File information structure and methods.
//!
//! This module provides the FileInfo struct which encapsulates file metadata
//! and provides convenient accessor methods for file properties.

use std::fs::{self, DirEntry, Metadata};
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// Represents information about a single file or directory.
/// Stores the path, file name, and metadata for efficient access.
pub struct FileInfo {
    pub path: PathBuf,      // Full path to the file
    pub file_name: String,  // File name (extracted from path for efficiency)
    pub metadata: Metadata, // File system metadata
}

impl FileInfo {
    /// Creates a FileInfo from a DirEntry (used when iterating directory contents)
    /// Note: Takes DirEntry by value because its methods consume self
    #[allow(clippy::needless_pass_by_value)]
    pub fn from_entry(entry: DirEntry) -> std::io::Result<Self> {
        let metadata = entry.metadata()?;
        let path = entry.path();
        let file_name = entry.file_name().to_string_lossy().to_string();
        Ok(FileInfo {
            path,
            file_name,
            metadata,
        })
    }

    /// Creates a FileInfo from a Path (used for single file listings)
    pub fn from_path(path: &Path) -> std::io::Result<Self> {
        // Use symlink_metadata to detect symlinks properly
        let metadata = fs::symlink_metadata(path)?;
        let file_name = path
            .file_name()
            .unwrap_or_else(|| std::ffi::OsStr::new("."))
            .to_string_lossy()
            .to_string();
        Ok(FileInfo {
            path: path.to_path_buf(),
            file_name,
            metadata,
        })
    }

    /// Returns modification time for sorting
    pub fn modified_time(&self) -> SystemTime {
        self.metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH)
    }

    /// Returns the block count for the file
    pub fn blocks(&self) -> u64 {
        #[cfg(unix)]
        {
            self.metadata.blocks()
        }
        #[cfg(not(unix))]
        {
            // Fallback for non-Unix: estimate blocks from size
            self.metadata.len().div_ceil(512)
        }
    }

    /// Check if file is hidden (starts with .)
    pub fn is_hidden(&self) -> bool {
        self.file_name.starts_with('.')
    }

    /// Check if this is a directory
    pub fn is_dir(&self) -> bool {
        self.metadata.is_dir()
    }

    /// Get symlink target if this is a symlink
    pub fn symlink_target(&self) -> Option<PathBuf> {
        if self.metadata.is_symlink() {
            fs::read_link(&self.path).ok()
        } else {
            None
        }
    }

    /// Get file size
    pub fn size(&self) -> u64 {
        self.metadata.len()
    }

    /// Get user ID
    pub fn uid(&self) -> u32 {
        self.metadata.uid()
    }

    /// Get group ID
    pub fn gid(&self) -> u32 {
        self.metadata.gid()
    }

    /// Get number of hard links
    pub fn nlink(&self) -> u64 {
        self.metadata.nlink()
    }

    /// Get modified time
    pub fn modified(&self) -> io::Result<SystemTime> {
        self.metadata.modified()
    }
}

use std::io;
