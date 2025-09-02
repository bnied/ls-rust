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

    #[arg(value_name = "PATH", default_value = ".", help = "Directory to list")]
    path: PathBuf,
}

struct FileInfo {
    entry: DirEntry,
    metadata: Metadata,
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
    let path = &args.path;
    
    if path.is_file() {
        let metadata = fs::metadata(path)?;
        display_file(path, &metadata, args);
    } else {
        let mut entries = collect_entries(path, args.all)?;
        entries.sort_by(|a, b| a.entry.file_name().cmp(&b.entry.file_name()));
        
        for file_info in entries {
            display_entry(&file_info, args);
        }
    }
    
    Ok(())
}

// Reads a directory and collects file information, optionally including hidden files
fn collect_entries(dir: &Path, show_all: bool) -> std::io::Result<Vec<FileInfo>> {
    let mut entries = Vec::new();
    
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();
        
        if !show_all && file_name_str.starts_with('.') {
            continue;
        }
        
        let metadata = entry.metadata()?;
        entries.push(FileInfo { entry, metadata });
    }
    
    Ok(entries)
}

// Displays a directory entry based on the provided flags (long format, size, etc.)
fn display_entry(file_info: &FileInfo, args: &Args) {
    if args.long {
        let name = file_info.entry.file_name().to_string_lossy().to_string();
        display_long_format(&file_info.metadata, &name, args);
    } else if args.size {
        let size = if args.human_readable {
            format_size_human(file_info.metadata.len())
        } else {
            format!("{:>8}", (file_info.metadata.blocks() * 512 + 1023) / 1024)
        };
        let colored_name = colorize_name(&file_info.entry.file_name().to_string_lossy(), &file_info.metadata);
        print!("{} ", size);
        println!("{}", colored_name);
    } else {
        let colored_name = colorize_name(&file_info.entry.file_name().to_string_lossy(), &file_info.metadata);
        println!("{}", colored_name);
    }
}

// Displays a single file (when path points to a file rather than directory)
fn display_file(path: &Path, metadata: &Metadata, args: &Args) {
    let file_name = path.file_name()
        .unwrap_or_else(|| std::ffi::OsStr::new("."))
        .to_string_lossy()
        .to_string();
    
    if args.long {
        display_long_format(metadata, &file_name, args);
    } else if args.size {
        let size = if args.human_readable {
            format_size_human(metadata.len())
        } else {
            format!("{:>8}", (metadata.blocks() * 512 + 1023) / 1024)
        };
        let colored_name = colorize_name(&file_name, metadata);
        println!("{} {}", size, colored_name);
    } else {
        let colored_name = colorize_name(&file_name, metadata);
        println!("{}", colored_name);
    }
}

// Formats and prints file information in long format (like ls -l)
fn display_long_format(metadata: &Metadata, name: &str, args: &Args) {
    let permissions = format_permissions(metadata);
    let nlink = metadata.nlink();
    let owner = get_user_by_uid(metadata.uid())
        .map(|u| u.name().to_string_lossy().to_string())
        .unwrap_or_else(|| metadata.uid().to_string());
    let group = get_group_by_gid(metadata.gid())
        .map(|g| g.name().to_string_lossy().to_string())
        .unwrap_or_else(|| metadata.gid().to_string());
    let size = if args.human_readable {
        format_size_human(metadata.len())
    } else {
        metadata.len().to_string()
    };
    let modified = format_time(metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH));
    let colored_name = colorize_name(name, metadata);
    
    println!("{} {:>3} {} {} {:>8} {} {}",
        permissions, nlink, owner, group, size, modified, colored_name);
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
    
    format!("{}{}{}{}", file_type, user, group, other)
}

// Converts a 3-bit permission value to rwx format
fn triplet(mode: u32) -> String {
    let r = if mode & 0b100 != 0 { 'r' } else { '-' };
    let w = if mode & 0b010 != 0 { 'w' } else { '-' };
    let x = if mode & 0b001 != 0 { 'x' } else { '-' };
    format!("{}{}{}", r, w, x)
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