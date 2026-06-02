# Chronicler

A Rust CLI tool for managing chronicle markdown files with automatic YAML frontmatter generation.

## Overview

`chronicler` is a command-line utility that processes chronicle markdown files (formatted as `chronicle-YYYY-MM-DD.md`) and automatically adds YAML frontmatter headers to create a linked sequence of journal entries. Each file gets a header with references to the previous entry and a journal link.

## Features

- Automatically adds YAML frontmatter headers to chronicle markdown files
- Links entries chronologically with `prev` references
- Validates existing headers to avoid duplicate processing
- Configurable chronicle directory via TOML config file
- Only processes files matching the `chronicle-YYYY-MM-DD.md` pattern

## Installation

### Prerequisites

- Rust 2024 edition or later
- Cargo package manager

### Building from Source

```bash
cargo build --release
```

The compiled binary will be available at `target/release/chronicler`.

## Configuration

Create a configuration file at `~/.config/chronicler/config.toml`:

```toml
chronicler_directory = "/path/to/your/chronicle/directory"
```

The `chronicler_directory` must be set and point to a valid directory containing your chronicle markdown files.

## Usage

### Update Headers

To process all chronicle markdown files in your configured directory:

```bash
chronicler --update-headers
```

Or use the short flag:

```bash
chronicler -u
```

### What It Does

When you run the update headers command, `chronicler`:

1. Reads your configured `chronicler_directory`
2. Finds all files matching the pattern `chronicle-YYYY-MM-DD.md`
3. Sorts them chronologically
4. Adds YAML frontmatter to each file with:
   - `prev`: Link to the previous chronicle entry (empty for the first file)
   - `journal`: Link in the format `[[YYYY-MM-DD]]`

### Example Output

For a file named `chronicle-2026-03-04.md` that is the second entry (after `chronicle-2026-03-03.md`):

```markdown
---
prev: "[[chronicle-2026-03-03]]"
journal: "[[2026-03-04]]"
---

Your original content here...
```

For the first entry in the sequence:

```markdown
---
prev: ""
journal: "[[2026-03-01]]"
---

Your original content here...
```

## File Naming Requirements

Files must follow the exact pattern: `chronicle-YYYY-MM-DD.md`

- Must start with `chronicle-`
- Followed by a date in `YYYY-MM-DD` format
- Must end with `.md` extension (lowercase)
- Date components must be zero-padded (e.g., `2026-03-04`, not `2026-3-4`)

Other markdown files in the directory (like `notes.md` or `2026-03-04.md`) will be ignored.

## Error Handling

The tool will exit with an error if:

- The config file is missing or malformed
- `chronicler_directory` is not set or is empty
- The specified directory doesn't exist
- No `chronicle-YYYY-MM-DD.md` files are found in the directory

## Development

### Running Tests

```bash
cargo test
```

### Project Structure

- `src/main.rs`: CLI entry point and configuration handling
- `src/lib.rs`: Core functionality for header parsing and file processing
- `tests/`: Test fixtures and integration tests
