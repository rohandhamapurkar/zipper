# Zipper

A secure file and directory compression utility written in Rust that creates AES-256 encrypted ZIP archives.

## Overview

Zipper is a command-line tool designed to simplify the creation of password-protected ZIP archives. Whether you need to protect a single sensitive file or an entire directory structure, Zipper makes it easy to create secure, encrypted archives that can only be accessed with the correct password.

## Features

- **Strong Encryption**: Uses AES-256 encryption for maximum security
- **Password Protection**: Requires a confirmed password for all archives
- **File & Directory Support**: Works with both individual files and entire directory structures
- **Cross-Platform**: Compatible with Windows, macOS, and Linux
- **Custom Naming**: Add optional prefix to generated ZIP files
- **Efficient Streaming**: Uses buffered streaming for memory-efficient compression of large files
- **Command-Line Interface**: Simple, interactive command-line experience

## Installation

### Pre-built Binaries

Download the latest pre-built binary for your platform from the [Releases](https://github.com/rohandhamapurkar/zipper/releases) page.

Available platforms:
- Windows (x64)
- macOS (Intel and Apple Silicon)
- Linux (x64)

### From Source

1. Clone the repository:
   ```bash
   git clone https://github.com/rohandhamapurkar/zipper.git
   cd zipper
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. The compiled binary will be available at `target/release/zipper`

### From Cargo

```bash
cargo install zipper
```

## Usage

### Basic Usage

To create an encrypted ZIP file from a file or directory:

```bash
zipper /path/to/file_or_directory
```

You'll be prompted to:
1. Enter an optional prefix for the output filename
2. Create and confirm a password for encryption

### Examples

**Zipping a single file:**
```bash
zipper important_document.pdf
# Creates important_document.zip with password protection
```

**Zipping a directory:**
```bash
zipper /path/to/project_folder
# Creates project_folder.zip containing all files and subdirectories
```

**Using a prefix:**
```bash
zipper important_document.pdf
# When prompted for prefix, enter "backup_2025"
# Creates backup_2025_important_document.zip
```

## Security

- **Encryption**: Uses the AES-256 encryption standard, which is widely recognized as secure
- **Password Storage**: Passwords are never stored, only used at runtime and then discarded
- **Memory Safety**: Built with Rust's memory safety guarantees to prevent security vulnerabilities

## How It Works

1. **Path Validation**: Verifies the provided path exists and determines if it's a file or directory
2. **Password Creation**: Securely collects and confirms the encryption password
3. **Archive Creation**: Initializes a new ZIP archive with the appropriate name
4. **File Processing**:
   - For single files: Adds the file directly to the archive
   - For directories: Recursively walks the directory structure, preserving paths
5. **Encryption**: Applies AES-256 encryption to all content using the provided password
6. **Finalization**: Properly finalizes the ZIP structure and saves the output

## Development

### Prerequisites

- Rust 2021 edition or newer
- Cargo package manager

### Dependencies

- `zip`: Handles ZIP archive creation and encryption
- `walkdir`: Efficiently traverses directory structures
- `rpassword`: Securely reads passwords without echoing to the terminal

### Building and Testing

```bash
# Build the project
cargo build

# Run tests
cargo test

# Build optimized release version
cargo build --release
```

### CI/CD Pipeline

This project uses GitHub Actions for continuous integration and deployment:

- Automatically builds binaries for multiple platforms (Windows, macOS, Linux)
- Creates releases when new version tags are pushed
- Supports both x64 and ARM64 architectures (Apple Silicon)

To create a new release:
1. Tag the commit with a version (e.g., `git tag v1.0.0`)
2. Push the tag to GitHub (`git push origin v1.0.0`)
3. The workflow will automatically build binaries and create a release

## License

```
MIT License

Copyright (c) 2025 Rohan Dhamapurkar

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

## Compatibility

ZIP files created with this tool are compatible with standard archive managers that support AES encryption, including:
- 7-Zip
- WinRAR
- The Unarchiver (macOS)
- Many others with AES-256 support

## Note

Password-protected ZIP files are only as secure as the password you choose. Always use strong, unique passwords for sensitive data.
