/*
 * Program to reimplement "ls" in Rust.
 */

use clap::Parser;
use chrono::{DateTime, Local};
use colored::*;
use std::fs::{self, DirEntry, Metadata};
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use users::{get_group_by_gid, get_user_by_uid};

#[derive(Parser, Debug)]
#[command(name = "ls")]
#[command(about = "List directory contents", long_about = None)]
#[command(disable_help_flag = true)]
struct Args {
    #[arg(short = 'l', long, help = "Use a long listing format")]
    long: bool,

    #[arg(short = 'a', long, help = "Show hidden files (starting with .)")]
    all: bool,

    #[arg(short = 's', long, help = "Show file sizes")]
    size: bool,

    #[arg(short = 'h', long = "human-readable", help = "Human-readable sizes")]
    human_readable: bool,

    #[arg(long, help = "Print help", action = clap::ArgAction::Help)]
    help: Option<bool>,

    #[arg(value_name = "PATH", help = "Paths to list")]
    paths: Vec<PathBuf>,
}

struct FileInfo {
    path: PathBuf,
    metadata: Metadata,
}

impl FileInfo {
    // Creates a FileInfo from a DirEntry (used when iterating directory contents)
    fn from_entry(entry: DirEntry) -> std::io::Result<Self> {
        let metadata = entry.metadata()?;
        let path = entry.path();
        Ok(FileInfo { path, metadata })
    }

    // Creates a FileInfo from a Path (used for single file listings)
    fn from_path(path: &Path) -> std::io::Result<Self> {
        let metadata = fs::metadata(path)?;
        Ok(FileInfo {
            path: path.to_path_buf(),
            metadata,
        })
    }

    // Extracts the file name from the path, defaults to "." if no name exists
    fn file_name(&self) -> String {
        self.path
            .file_name()
            .unwrap_or_else(|| std::ffi::OsStr::new("."))
            .to_string_lossy()
            .to_string()
    }

    // Displays the file entry based on command-line arguments (long, size, or normal format)
    fn display(&self, args: &Args) {
        let name = self.file_name();

        if args.long {
            self.display_long_format(&name, args);
        } else if args.size {
            let size = if args.human_readable {
                format_size_human(self.metadata.len())
            } else {
                format_block_size(&self.metadata)
            };
            let colored_name = colorize_name(&name, &self.metadata);
            println!("{size} {colored_name}");
        } else {
            let colored_name = colorize_name(&name, &self.metadata);
            println!("{colored_name}");
        }
    }

    // Formats and prints file information in long format (like ls -l)
    fn display_long_format(&self, name: &str, args: &Args) {
        let permissions = format_permissions(&self.metadata);
        let nlink = self.metadata.nlink();
        let owner = get_user_by_uid(self.metadata.uid())
            .map(|u| u.name().to_string_lossy().to_string())
            .unwrap_or_else(|| self.metadata.uid().to_string());
        let group = get_group_by_gid(self.metadata.gid())
            .map(|g| g.name().to_string_lossy().to_string())
            .unwrap_or_else(|| self.metadata.gid().to_string());
        let size = if args.human_readable {
            format_size_human(self.metadata.len())
        } else {
            self.metadata.len().to_string()
        };
        let modified = format_time(self.metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH));
        let colored_name = colorize_name(name, &self.metadata);

        println!("{permissions} {nlink:>3} {owner} {group} {size:>8} {modified} {colored_name}");
    }
}

fn main() {
    let args = Args::parse();

    if let Err(e) = run(&args) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

// Main entry point for the ls logic, handles both file and directory listing
fn run(args: &Args) -> std::io::Result<()> {
    let paths = if args.paths.is_empty() {
        vec![PathBuf::from(".")]
    } else {
        args.paths.clone()
    };

    let multiple_paths = paths.len() > 1;

    for (index, path) in paths.iter().enumerate() {
        if multiple_paths && index > 0 {
            println!();
        }

        if multiple_paths {
            println!("{}:", path.display());
        }

        if path.is_file() {
            let file_info = FileInfo::from_path(path)?;
            file_info.display(args);
        } else {
            let mut entries = collect_entries(path, args.all)?;
            entries.sort_by(|a, b| {
                a.file_name().to_lowercase().cmp(&b.file_name().to_lowercase())
            });

            for file_info in entries {
                file_info.display(args);
            }
        }
    }

    Ok(())
}

// Reads a directory and collects file information, optionally including hidden files
fn collect_entries(dir: &Path, show_all: bool) -> std::io::Result<Vec<FileInfo>> {
    let mut entries = vec![];

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();

        if !show_all && file_name_str.starts_with('.') {
            continue;
        }

        entries.push(FileInfo::from_entry(entry)?);
    }

    Ok(entries)
}

// Converts Unix file permissions to the standard drwxrwxrwx format
fn format_permissions(metadata: &Metadata) -> String {
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

// Converts a 3-bit permission value to rwx format
fn triplet(mode: u32) -> String {
    let r = if mode & 0b100 != 0 { 'r' } else { '-' };
    let w = if mode & 0b010 != 0 { 'w' } else { '-' };
    let x = if mode & 0b001 != 0 { 'x' } else { '-' };
    format!("{r}{w}{x}")
}

// Formats a system time as a human-readable date string (e.g., "Jan 15 10:30")
fn format_time(system_time: SystemTime) -> String {
    let datetime: DateTime<Local> = system_time.into();
    datetime.format("%b %d %H:%M").to_string()
}

// Converts byte size to human-readable format (B, K, M, G, T, P)
fn format_size_human(size: u64) -> String {
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

// Formats block size for the -s flag, handling platform differences
fn format_block_size(metadata: &Metadata) -> String {
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

// Applies color to filename based on file type and permissions
fn colorize_name(name: &str, metadata: &Metadata) -> ColoredString {
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