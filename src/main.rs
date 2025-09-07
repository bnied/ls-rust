//! Main function - program entrypoint

mod directory;
mod file_info;
mod formatter;
mod sorting;
mod utils;

use clap::Parser;
use directory::{collect_entries, get_subdirectories};
use file_info::FileInfo;
use formatter::{FileInfoFormatter, Format};
use sorting::{sort_directories, sort_entries, SortConfig};
use std::io;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(name = "ls")]
#[command(about = "List directory contents", long_about = None)]
#[command(disable_help_flag = true)]
#[allow(clippy::struct_excessive_bools)]
pub struct Args {
    #[arg(short = 'l', long, help = "Use a long listing format")]
    pub long: bool,

    #[arg(short = 'a', long, help = "Show hidden files (starting with .)")]
    pub all: bool,

    #[arg(short = 's', long, help = "Show file sizes")]
    pub size: bool,

    #[arg(short = 'h', long = "human-readable", help = "Human-readable sizes")]
    pub human_readable: bool,

    #[arg(short = 'R', long, help = "List subdirectories recursively")]
    pub recursive: bool,

    #[arg(short = 't', long, help = "Sort by modification time, newest first")]
    pub time: bool,

    #[arg(short = 'r', long, help = "Reverse order while sorting")]
    pub reverse: bool,

    #[arg(short = '1', long = "one", help = "List one file per line")]
    pub one: bool,

    #[arg(long, help = "Print help", action = clap::ArgAction::Help)]
    pub help: Option<bool>,

    #[arg(value_name = "PATH", help = "Paths to list")]
    pub paths: Vec<PathBuf>,
}

impl Args {
    /// Main entry point for the ls logic.
    /// Processes all provided paths (or current directory if none specified).
    /// Collects and reports errors at the end rather than failing immediately.
    ///
    /// # Errors
    /// Returns an error if critical I/O operations fail.
    pub fn run(&self) -> io::Result<()> {
        let paths = if self.paths.is_empty() {
            vec![PathBuf::from(".")]
        } else {
            self.paths.clone()
        };

        let multiple_paths = paths.len() > 1;
        let mut errors = Vec::new();

        for (index, path) in paths.iter().enumerate() {
            if multiple_paths && index > 0 {
                println!();
            }

            if let Err(e) = self.list_path(path, multiple_paths, 0) {
                errors.push((path.clone(), e));
            }
        }

        // Report errors at the end
        for (path, error) in errors {
            eprintln!("ls: {}: {}", path.display(), error);
        }

        Ok(())
    }

    /// Lists a single path (file or directory).
    /// Handles both file and directory listing, with support for recursive traversal.
    ///
    /// # Arguments
    /// * `path` - The path to list
    /// * `show_path_header` - Whether to print the path name before listing
    /// * `depth` - Current recursion depth (used for recursive listing)
    fn list_path(&self, path: &Path, show_path_header: bool, depth: usize) -> io::Result<()> {
        if path.is_file() {
            let file_info = FileInfo::from_path(path)?;
            self.display_file(&file_info);
        } else {
            if show_path_header || (self.recursive && depth > 0) {
                println!("{}:", path.display());
            }

            // Collect and sort entries
            let mut entries = collect_entries(path, self.all)?;
            let sort_config = SortConfig::new(self.time, self.reverse);
            sort_entries(&mut entries, &sort_config);

            // Display total blocks for long format
            if self.long && !entries.is_empty() {
                let total = entries.iter().map(|f| f.blocks() * 512 / 1024).sum::<u64>();
                println!("total {total}");
            }

            // Display each entry
            for file_info in &entries {
                self.display_file(file_info);
            }

            // Handle recursive listing
            if self.recursive {
                self.list_subdirectories(&entries, depth)?;
            }
        }

        Ok(())
    }

    /// Recursively lists subdirectories
    fn list_subdirectories(&self, entries: &[FileInfo], depth: usize) -> io::Result<()> {
        let mut dirs = get_subdirectories(entries);

        // Sort directories for consistent output
        sort_directories(&mut dirs);

        for dir in dirs {
            println!();
            if let Err(e) = self.list_path(&dir.path, true, depth + 1) {
                eprintln!("ls: {}: {}", dir.path.display(), e);
            }
        }

        Ok(())
    }

    /// Displays a single file using the appropriate format.
    /// Creates a FileInfoFormatter with the correct format and renders it.
    fn display_file(&self, file_info: &FileInfo) {
        let format = self.get_format();
        let formatter = FileInfoFormatter {
            file_info,
            format,
            human_readable: self.human_readable,
        };
        println!("{formatter}");
    }

    /// Determines the display format based on command-line arguments.
    /// Priority: -1 (one column) > -l (long) > -s (with size) > default (name only)
    fn get_format(&self) -> Format {
        if self.one {
            Format::Name
        } else if self.long {
            Format::Long
        } else if self.size {
            Format::WithSize
        } else {
            Format::Name
        }
    }
}

fn main() {
    let args = Args::parse();

    if let Err(e) = args.run() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
