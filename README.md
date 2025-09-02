# ls-rust

A Rust implementation of the Unix `ls` command with support for common flags.

## Features

- List directory contents with various formatting options
- Support for hidden files
- Long format with detailed file information
- File size display in blocks or human-readable format
- Unix permissions, ownership, and timestamp display

## Installation

### From Source

```bash
git clone https://github.com/yourusername/ls-rust.git
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

## Permissions Format

The permissions string in long format follows Unix conventions:

- First character: file type (`-` regular, `d` directory, `l` symlink)
- Next 9 characters: permissions in groups of 3 (owner, group, other)
  - `r`: read permission
  - `w`: write permission
  - `x`: execute permission
  - `-`: permission not granted

Example: `-rw-r--r--` means a regular file with read/write for owner, read for group, and read for others.

## Dependencies

- `clap`: Command-line argument parsing
- `chrono`: Date and time formatting
- `users`: User and group name resolution

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
```bash
cargo test
```

## License

BSD 2-Clause License

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Comparison with GNU ls

This implementation covers the most commonly used features of GNU `ls`. Some advanced features not yet implemented include:

- Recursive listing (`-R`)
- Sorting options (by time, size, etc.)
- Color output
- Extended attributes display
- SELinux context

## Author

Your Name

## Acknowledgments

Inspired by the GNU coreutils `ls` command.