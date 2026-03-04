//! Core chronicle file operations.
//!
//! Handles appending entries and updating headers for chronicle-YYYY-MM-DD.md files.

use crate::{extract_between_dashes, list_mds, Header};
use chrono::{Duration, Local};
use std::fs::{self, File};
use std::io::{self, ErrorKind, Write};
use std::path::{Path, PathBuf};

/// Appends a timestamped entry to today's chronicle file.
///
/// If the file doesn't exist, creates it with a YAML frontmatter header.
/// The entry is formatted as a markdown list item with the current time.
pub fn append_chronicle_entry(chronicler_directory: &Path, entry_text: &str) -> io::Result<()> {
    let local_now = Local::now();
    let time = local_now.format("%H:%M");
    let date = local_now.date_naive();
    let yesterday = (local_now - Duration::days(1)).date_naive();

    let path = chronicler_directory.join(format!("chronicle-{date}.md"));

    // If file doesn't exist, create it with YAML frontmatter header
    if !path.exists() {
        let mut file = File::options().append(true).create(true).open(&path)?;

        // Create YAML frontmatter header
        let header = Header {
            prev: format!("[[chronicle-{yesterday}]]"),
            journal: Some(format!("[[{date}]]")),
        };

        writeln!(&mut file, "{}", header)?;
        writeln!(&mut file)?;
        writeln!(&mut file, "## Chronicles")?;
        writeln!(&mut file)?;
    }

    // Append the entry in markdown list format
    let mut file = File::options().append(true).open(&path)?;

    writeln!(&mut file, "- {time}: {entry_text}")?;

    Ok(())
}

/// Updates headers for all chronicle-YYYY-MM-DD.md files in a directory.
///
/// Sets the `prev` field to link to the previous chronologically-ordered file.
/// The first file will have an empty `prev` field.
pub fn update_chronicler_headers(chronicler_directory: impl AsRef<Path>) -> io::Result<()> {
    let md_files = list_mds(chronicler_directory)?;
    let dated_md_files: Vec<PathBuf> = md_files
        .into_iter()
        .filter(|path| is_chronicler_markdown_filename(path))
        .collect();

    if dated_md_files.is_empty() {
        return Err(io::Error::new(
            ErrorKind::NotFound,
            "No chronicle-YYYY-MM-DD.md files found",
        ));
    }

    Ok(
        for (index, file_path) in dated_md_files.iter().enumerate() {
            let prev = if index == 0 {
                String::new()
            } else {
                let previous_date = chronicler_date_from_path(&dated_md_files[index - 1])?;

                format!("[[chronicle-{previous_date}]]")
            };

            println!("Processing: {}", file_path.display());
            add_chronicler_header(file_path, &prev)?;
        },
    )
}

fn is_chronicler_markdown_filename(path: &Path) -> bool {
    let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
        return false;
    };

    if !name.starts_with("chronicle-") || !name.ends_with(".md") {
        return false;
    }

    let date = &name["chronicle-".len()..name.len() - ".md".len()];
    if date.len() != 10 {
        return false;
    }

    let bytes = date.as_bytes();
    bytes[0..4].iter().all(u8::is_ascii_digit)
        && bytes[4] == b'-'
        && bytes[5..7].iter().all(u8::is_ascii_digit)
        && bytes[7] == b'-'
        && bytes[8..10].iter().all(u8::is_ascii_digit)
}

fn chronicler_date_from_path(path: &Path) -> io::Result<String> {
    let name = path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| {
            io::Error::new(
                ErrorKind::InvalidData,
                format!("Filename is not valid UTF-8: {}", path.display()),
            )
        })?;

    if !name.starts_with("chronicle-") || !name.ends_with(".md") {
        return Err(io::Error::new(
            ErrorKind::InvalidData,
            format!(
                "Filename does not match chronicle-YYYY-MM-DD.md: {}",
                path.display()
            ),
        ));
    }

    let date = &name["chronicle-".len()..name.len() - ".md".len()];
    Ok(date.to_string())
}

fn add_chronicler_header(
    file_path: impl AsRef<std::path::Path>,
    prev: &str,
) -> Result<(), std::io::Error> {
    let file_path = file_path.as_ref();
    let contents = match fs::read_to_string(file_path) {
        Ok(contents) => contents,
        Err(err) if err.kind() == ErrorKind::NotFound => {
            println!("File does not exist: {}", file_path.display());
            return Ok(());
        }
        Err(err) => return Err(err),
    };

    let current_date = chronicler_date_from_path(file_path)?;

    let header = Header {
        prev: prev.to_string(),
        journal: Some(format!("[[{current_date}]]")),
    };

    let updated_contents = if let Some(_existing_header) = extract_between_dashes(&contents) {
        // Header exists - replace it
        let header_start = contents.find("---").unwrap();
        let header_end_marker =
            contents[header_start + 3..].find("---").unwrap() + header_start + 3;
        let after_header = header_end_marker + 3;

        // Skip the newline after the closing --- if present
        let content_start = if contents.as_bytes().get(after_header) == Some(&b'\n') {
            after_header + 1
        } else {
            after_header
        };

        format!("{}\n{}", header, &contents[content_start..])
    } else {
        // No header - prepend it
        format!("{}\n{}", header, contents)
    };

    fs::write(file_path, updated_contents)?;

    if extract_between_dashes(&contents).is_some() {
        println!("Header updated.\n");
    } else {
        println!("Header added to file.\n");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{is_chronicler_markdown_filename, update_chronicler_headers};
    use crate::{extract_between_dashes, parse_header};
    use std::fs;
    use std::io::ErrorKind;
    use std::path::Path;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_temp_path(prefix: &str) -> std::path::PathBuf {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be after UNIX_EPOCH")
            .as_nanos();

        std::env::temp_dir().join(format!("{prefix}_{}_{}", std::process::id(), stamp))
    }

    #[test]
    fn chronicler_markdown_filename_matches_expected_pattern() {
        assert!(is_chronicler_markdown_filename(Path::new(
            "chronicle-2026-01-31.md"
        )));
    }

    #[test]
    fn chronicler_markdown_filename_rejects_non_matching_names() {
        let non_matches = [
            "notes.md",
            "chronicle-2026-1-31.md",
            "chronicle-2026-01-31.MD",
            "2026-01-31.md",
            "2026-01-31.txt",
            "chronicle-20260131.md",
            "chronicle-2026-01-31.md.bak",
        ];

        for file_name in non_matches {
            assert!(
                !is_chronicler_markdown_filename(Path::new(file_name)),
                "{file_name} should not match chronicle-YYYY-MM-DD.md"
            );
        }
    }

    #[test]
    fn update_chronicler_headers_only_updates_chronicler_markdown_files() {
        let dir = unique_temp_path("update_chronicler_headers_only_chronicler");
        fs::create_dir_all(&dir).expect("fixture directory should be created");

        let dated_file = dir.join("chronicle-2026-01-01.md");
        let non_dated_md = dir.join("notes.md");
        let plain_dated_md = dir.join("2026-01-02.md");

        fs::write(&dated_file, "Entry body\n").expect("dated fixture file should be written");
        fs::write(&non_dated_md, "Entry body\n")
            .expect("non-dated md fixture file should be written");
        fs::write(&plain_dated_md, "Entry body\n")
            .expect("plain dated md fixture file should be written");

        update_chronicler_headers(&dir).expect("updating headers should succeed");

        let dated_contents =
            fs::read_to_string(&dated_file).expect("dated file should be readable");
        let non_dated_contents =
            fs::read_to_string(&non_dated_md).expect("non-dated md should be readable");
        let plain_dated_contents =
            fs::read_to_string(&plain_dated_md).expect("plain dated md should be readable");

        assert!(extract_between_dashes(&dated_contents).is_some());
        assert_eq!(non_dated_contents, "Entry body\n");
        assert_eq!(plain_dated_contents, "Entry body\n");

        fs::remove_dir_all(&dir).expect("fixture directory should be cleaned up");
    }

    #[test]
    fn update_chronicler_headers_returns_not_found_when_no_chronicler_markdown_files() {
        let dir = unique_temp_path("update_chronicler_headers_no_chronicler_files");
        fs::create_dir_all(&dir).expect("fixture directory should be created");

        fs::write(dir.join("notes.md"), "Entry body\n")
            .expect("non-dated md fixture file should be written");
        fs::write(dir.join("2026-01-01.md"), "Entry body\n")
            .expect("plain dated md fixture file should be written");

        let err = update_chronicler_headers(&dir)
            .expect_err("should fail when no chronicler markdown files exist");
        assert_eq!(err.kind(), ErrorKind::NotFound);

        fs::remove_dir_all(&dir).expect("fixture directory should be cleaned up");
    }

    #[test]
    fn update_chronicler_headers_sets_prev_to_previous_dated_file() {
        let dir = unique_temp_path("update_chronicler_headers_prev_previous");
        fs::create_dir_all(&dir).expect("fixture directory should be created");

        let first = dir.join("chronicle-2026-02-01.md");
        let second = dir.join("chronicle-2026-02-02.md");
        fs::write(&first, "Entry body\n").expect("first fixture file should be written");
        fs::write(&second, "Entry body\n").expect("second fixture file should be written");

        update_chronicler_headers(&dir).expect("updating headers should succeed");

        let first_contents = fs::read_to_string(&first).expect("first file should be readable");
        let second_contents = fs::read_to_string(&second).expect("second file should be readable");

        let first_header = extract_between_dashes(&first_contents)
            .and_then(parse_header)
            .expect("first file should have parseable header");
        let second_header = extract_between_dashes(&second_contents)
            .and_then(parse_header)
            .expect("second file should have parseable header");

        assert_eq!(first_header.prev, "");
        assert_eq!(second_header.prev, "[[chronicle-2026-02-01]]");
        assert_eq!(first_header.journal.as_deref(), Some("[[2026-02-01]]"));
        assert_eq!(second_header.journal.as_deref(), Some("[[2026-02-02]]"));

        fs::remove_dir_all(&dir).expect("fixture directory should be cleaned up");
    }

    #[test]
    fn update_chronicler_headers_handles_date_gaps_for_prev() {
        let dir = unique_temp_path("update_chronicler_headers_prev_gap");
        fs::create_dir_all(&dir).expect("fixture directory should be created");

        let first = dir.join("chronicle-2026-02-01.md");
        let second = dir.join("chronicle-2026-02-09.md");
        fs::write(&first, "Entry body\n").expect("first fixture file should be written");
        fs::write(&second, "Entry body\n").expect("second fixture file should be written");

        update_chronicler_headers(&dir).expect("updating headers should succeed");

        let second_contents = fs::read_to_string(&second).expect("second file should be readable");

        let second_header = extract_between_dashes(&second_contents)
            .and_then(parse_header)
            .expect("second file should have parseable header");

        assert_eq!(second_header.prev, "[[chronicle-2026-02-01]]");

        fs::remove_dir_all(&dir).expect("fixture directory should be cleaned up");
    }

    #[test]
    fn append_chronicle_entry_creates_new_file_with_header() {
        let dir = unique_temp_path("append_creates_file");
        fs::create_dir_all(&dir).expect("fixture directory should be created");

        super::append_chronicle_entry(&dir, "First entry")
            .expect("appending to new file should succeed");

        let today = chrono::Local::now().date_naive();
        let file_path = dir.join(format!("chronicle-{today}.md"));
        assert!(file_path.exists(), "chronicle file should be created");

        let contents = fs::read_to_string(&file_path).expect("file should be readable");

        // Check that YAML header exists
        assert!(extract_between_dashes(&contents).is_some());
        let header = extract_between_dashes(&contents)
            .and_then(parse_header)
            .expect("file should have valid YAML header");

        assert_eq!(header.journal, Some(format!("[[{today}]]")));
        assert!(contents.contains("## Chronicles"));
        assert!(contents.contains("First entry"));

        fs::remove_dir_all(&dir).expect("fixture directory should be cleaned up");
    }

    #[test]
    fn append_chronicle_entry_appends_to_existing_file() {
        let dir = unique_temp_path("append_to_existing");
        fs::create_dir_all(&dir).expect("fixture directory should be created");

        super::append_chronicle_entry(&dir, "First entry").expect("first append should succeed");
        super::append_chronicle_entry(&dir, "Second entry").expect("second append should succeed");

        let today = chrono::Local::now().date_naive();
        let file_path = dir.join(format!("chronicle-{today}.md"));
        let contents = fs::read_to_string(&file_path).expect("file should be readable");

        assert!(contents.contains("First entry"));
        assert!(contents.contains("Second entry"));

        // Check that header only appears once
        let header_count = contents.matches("---").count();
        assert_eq!(
            header_count, 2,
            "should have exactly one YAML header (2 delimiters)"
        );

        fs::remove_dir_all(&dir).expect("fixture directory should be cleaned up");
    }

    #[test]
    fn append_chronicle_entry_uses_markdown_list_format() {
        let dir = unique_temp_path("append_markdown_format");
        fs::create_dir_all(&dir).expect("fixture directory should be created");

        super::append_chronicle_entry(&dir, "Test entry").expect("append should succeed");

        let today = chrono::Local::now().date_naive();
        let file_path = dir.join(format!("chronicle-{today}.md"));
        let contents = fs::read_to_string(&file_path).expect("file should be readable");

        // Check for markdown list format (starts with "- " followed by time and entry)
        assert!(
            contents
                .lines()
                .any(|line| line.starts_with("- ") && line.contains("Test entry")),
            "entry should be in markdown list format"
        );

        fs::remove_dir_all(&dir).expect("fixture directory should be cleaned up");
    }

    #[test]
    fn append_chronicle_entry_includes_yesterday_in_prev_link() {
        let dir = unique_temp_path("append_prev_link");
        fs::create_dir_all(&dir).expect("fixture directory should be created");

        super::append_chronicle_entry(&dir, "Test entry").expect("append should succeed");

        let today = chrono::Local::now().date_naive();
        let yesterday = (chrono::Local::now() - chrono::Duration::days(1)).date_naive();
        let file_path = dir.join(format!("chronicle-{today}.md"));
        let contents = fs::read_to_string(&file_path).expect("file should be readable");

        let header = extract_between_dashes(&contents)
            .and_then(parse_header)
            .expect("file should have valid header");

        assert_eq!(header.prev, format!("[[chronicle-{yesterday}]]"));

        fs::remove_dir_all(&dir).expect("fixture directory should be cleaned up");
    }
}
