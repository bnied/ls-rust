# ls-rust

A Rust implementation of the Unix `ls` command with support for common flags.

## Features

- List directory contents with various formatting options
- Support for hidden files (`-a`)
- Long format with detailed file information (`-l`)
- File size display in blocks or human-readable format (`-s`, `-h`)
- Recursive directory listing (`-R`)
- Sort by modification time (`-t`)
- Reverse sort order (`-r`)
- One file per line output (`-1`)
- Unix permissions, ownership, and timestamp display
- Colored output for different file types (directories, executables, symlinks)
- Symlink target display in long format
- Multiple path support with proper headers
- Error resilience (continues on errors, reports at end)

## Installation

### From Source

```bash
git clone https://github.com/bnied/ls-rust.git
cd ls-rust
cargo build --release
```

The binary will be available at `./target/release/ls-rust`

### Using Cargo

```bash
cargo install --path .
```

## Usage

```bash
ls-rust [OPTIONS] [PATH]
```

### Arguments

- `PATH`: Directory or file to list (default: current directory)

### Options

- `-l, --long`: Use long listing format showing permissions, links, owner, group, size, and modification time
- `-a, --all`: Show all files including hidden files (those starting with `.`)
- `-s, --size`: Display file sizes in 1K blocks
- `-h, --human-readable`: Show file sizes in human-readable format (B, K, M, G, etc.)
- `-R, --recursive`: List subdirectories recursively
- `-t, --time`: Sort by modification time, newest first
- `-r, --reverse`: Reverse order while sorting
- `-1, --one`: List one file per line
- `--help`: Display help message

### Examples

List files in current directory:
```bash
ls-rust
```

List all files including hidden:
```bash
ls-rust -a
```

Long format listing:
```bash
ls-rust -l
```

Long format with human-readable sizes:
```bash
ls-rust -lh
```

Show file sizes in blocks:
```bash
ls-rust -s
```

Combine multiple flags:
```bash
ls-rust -las
```

List specific directory:
```bash
ls-rust -la /usr/local
```

List recursively:
```bash
ls-rust -R src
```

Sort by time, newest first:
```bash
ls-rust -lt
```

Sort by time, oldest first:
```bash
ls-rust -ltr
```

One file per line:
```bash
ls-rust -1
```

List multiple paths:
```bash
ls-rust src tests
```

## Output Format

### Standard Output
```
file1.txt
file2.rs
directory/
```

### Long Format (`-l`)
```
-rw-r--r--   1 user group    1234 Jan 15 10:30 file.txt
drwxr-xr-x   3 user group    4096 Jan 15 09:45 directory
```

### With Size (`-s`)
```
   4 file1.txt
  12 file2.rs
   4 directory
```

### Human-Readable Sizes (`-lh`)
```
-rw-r--r--   1 user group    1.2K Jan 15 10:30 file.txt
-rw-r--r--   1 user group    5.4M Jan 14 14:22 large_file.zip
```

### Recursive Listing (`-R`)
```
src:
file_info.rs
formatter.rs
main.rs

src/tests:
test_utils.rs
```

### Multiple Paths
```
src:
main.rs
utils.rs

tests:
cli_test.rs
output_test.rs
```

## Permissions Format

The permissions string in long format follows Unix conventions:

- First character: file type (`-` regular, `d` directory, `l` symlink)
- Next 9 characters: permissions in groups of 3 (owner, group, other)
  - `r`: read permission
  - `w`: write permission
  - `x`: execute permission
  - `-`: permission not granted

Example: `-rw-r--r--` means a regular file with read/write for owner, read for group, and read for others.

## Project Structure

The codebase follows a modular architecture:

```
src/
├── main.rs         # Entry point and CLI argument handling
├── file_info.rs    # FileInfo struct for file metadata
├── formatter.rs    # Display formatting with FileInfoFormatter
├── directory.rs    # Directory traversal and entry collection
├── sorting.rs      # Sorting configuration and implementation
└── utils.rs        # Utility functions for formatting and colors

tests/
├── cli_test.rs     # Integration tests for CLI functionality
└── output_test.rs  # Integration tests for output formatting
```

## Dependencies

- `clap`: Command-line argument parsing
- `chrono`: Date and time formatting
- `users`: User and group name resolution
- `colored`: Terminal color output

### Development Dependencies

- `tempfile`: Creating temporary files and directories for testing
- `assert_cmd`: Command-line application testing
- `predicates`: Assertions and predicates for testing

## Building

### Debug Build
```bash
cargo build
```

### Release Build
```bash
cargo build --release
```

### Run Tests

Run all tests (unit and integration):
```bash
cargo test
```

Run only unit tests:
```bash
cargo test --lib
```

Run only integration tests:
```bash
cargo test --test '*'
```

Run with verbose output:
```bash
cargo test -- --nocapture
```

### Code Quality

Check with Clippy:
```bash
cargo clippy -- -W clippy::pedantic
```

Format code:
```bash
cargo fmt
```

Check formatting:
```bash
cargo fmt -- --check
```

## License

BSD 2-Clause License. See [LICENSE](./LICENSE) for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Comparison with GNU ls

This implementation covers the most commonly used features of GNU `ls`:

### Implemented Features
- Basic file listing
- Hidden files (`-a`)
- Long format (`-l`)
- File sizes (`-s`)
- Human-readable sizes (`-h`)
- Recursive listing (`-R`)
- Time-based sorting (`-t`)
- Reverse sorting (`-r`)
- One file per line (`-1`)
- Colored output for file types
- Symlink target display
- Multiple path support

### Features Not Yet Implemented
- Sorting by size
- Directory-first sorting
- Extended attributes display
- SELinux context
- Additional time display options (access time, creation time)
- File type indicators (`-F`)
- Quoted output (`-Q`)

## Acknowledgments

Inspired by the GNU coreutils `ls` command.