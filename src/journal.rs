//! Core journal file operations.
//!
//! Handles updating headers for YYYY-MM-DD.md journal files in Obsidian directories.

use crate::{extract_between_dashes, list_mds, parse_header, Header};
use std::fs;
use std::io::{self, ErrorKind};
use std::path::{Path, PathBuf};

/// Updates headers for all YYYY-MM-DD.md journal files in a directory.
///
/// Sets the `prev` field to link to the previous chronologically-ordered file.
/// The first file will have an empty `prev` field.
/// Journal files do not include a `journal` field in their headers.
pub fn update_journal_headers(journal_directory: impl AsRef<Path>) -> io::Result<()> {
    let md_files = list_mds(journal_directory)?;
    let dated_md_files: Vec<PathBuf> = md_files
        .into_iter()
        .filter(|path| is_journal_markdown_filename(path))
        .collect();

    if dated_md_files.is_empty() {
        return Err(io::Error::new(
            ErrorKind::NotFound,
            "No YYYY-MM-DD.md journal files found",
        ));
    }

    Ok(
        for (index, file_path) in dated_md_files.iter().enumerate() {
            let prev = if index == 0 {
                String::new()
            } else {
                let previous_date = journal_date_from_path(&dated_md_files[index - 1])?;

                format!("[[{previous_date}]]")
            };

            println!("Processing: {}", file_path.display());
            add_journal_header(file_path, &prev)?;
        },
    )
}

fn is_journal_markdown_filename(path: &Path) -> bool {
    let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
        return false;
    };

    if !name.ends_with(".md") {
        return false;
    }

    let date = &name[..name.len() - ".md".len()];
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

fn journal_date_from_path(path: &Path) -> io::Result<String> {
    let name = path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| {
            io::Error::new(
                ErrorKind::InvalidData,
                format!("Filename is not valid UTF-8: {}", path.display()),
            )
        })?;

    if !name.ends_with(".md") {
        return Err(io::Error::new(
            ErrorKind::InvalidData,
            format!("Filename does not match YYYY-MM-DD.md: {}", path.display()),
        ));
    }

    let date = &name[..name.len() - ".md".len()];
    Ok(date.to_string())
}

fn add_journal_header(
    file_path: impl AsRef<std::path::Path>,
    prev: &str,
) -> Result<(), std::io::Error> {
    let file_path = file_path.as_ref();
    let contents = match fs::read_to_string(file_path) {
        Ok(contents) => contents,
        Err(err) if err.kind() == ErrorKind::NotFound => {
            println!("File does not exist. Skipping.\n");
            return Ok(());
        }
        Err(err) => return Err(err),
    };
    let has_valid_header = extract_between_dashes(&contents)
        .and_then(parse_header)
        .is_some();
    if has_valid_header {
        println!("Valid header already exists. No changes made.\n");
        return Ok(());
    }

    let header = Header {
        prev: prev.to_string(),
        journal: None,
    };
    let updated_contents = format!("{}\n{}", header, contents);
    fs::write(file_path, updated_contents)?;

    println!("Header added to file.\n");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{is_journal_markdown_filename, update_journal_headers};
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
    fn journal_markdown_filename_matches_expected_pattern() {
        assert!(is_journal_markdown_filename(Path::new("2026-01-31.md")));
    }

    #[test]
    fn journal_markdown_filename_rejects_non_matching_names() {
        let non_matches = [
            "notes.md",
            "2026-1-31.md",
            "2026-01-31.MD",
            "2026-01-31.txt",
            "20260131.md",
            "2026-01-31.md.bak",
            "chronicle-2026-01-31.md",
        ];

        for file_name in non_matches {
            assert!(
                !is_journal_markdown_filename(Path::new(file_name)),
                "{file_name} should not match YYYY-MM-DD.md"
            );
        }
    }

    #[test]
    fn update_journal_headers_only_updates_journal_markdown_files() {
        let dir = unique_temp_path("update_journal_headers_only_journal");
        fs::create_dir_all(&dir).expect("fixture directory should be created");

        let dated_file = dir.join("2026-01-01.md");
        let non_dated_md = dir.join("notes.md");
        let chronicle_dated_md = dir.join("chronicle-2026-01-02.md");

        fs::write(&dated_file, "Entry body\n").expect("dated fixture file should be written");
        fs::write(&non_dated_md, "Entry body\n")
            .expect("non-dated md fixture file should be written");
        fs::write(&chronicle_dated_md, "Entry body\n")
            .expect("chronicle dated md fixture file should be written");

        update_journal_headers(&dir).expect("updating headers should succeed");

        let dated_contents =
            fs::read_to_string(&dated_file).expect("dated file should be readable");
        let non_dated_contents =
            fs::read_to_string(&non_dated_md).expect("non-dated md should be readable");
        let chronicle_dated_contents =
            fs::read_to_string(&chronicle_dated_md).expect("chronicle dated md should be readable");

        assert!(extract_between_dashes(&dated_contents).is_some());
        assert_eq!(non_dated_contents, "Entry body\n");
        assert_eq!(chronicle_dated_contents, "Entry body\n");

        fs::remove_dir_all(&dir).expect("fixture directory should be cleaned up");
    }

    #[test]
    fn update_journal_headers_returns_not_found_when_no_journal_files() {
        let dir = unique_temp_path("update_journal_headers_no_journal_files");
        fs::create_dir_all(&dir).expect("fixture directory should be created");

        fs::write(dir.join("notes.md"), "Entry body\n")
            .expect("non-dated md fixture file should be written");
        fs::write(dir.join("chronicle-2026-01-01.md"), "Entry body\n")
            .expect("chronicle dated md fixture file should be written");

        let err = update_journal_headers(&dir)
            .expect_err("should fail when no journal markdown files exist");
        assert_eq!(err.kind(), ErrorKind::NotFound);

        fs::remove_dir_all(&dir).expect("fixture directory should be cleaned up");
    }

    #[test]
    fn update_journal_headers_sets_prev_to_previous_dated_file() {
        let dir = unique_temp_path("update_journal_headers_prev_previous");
        fs::create_dir_all(&dir).expect("fixture directory should be created");

        let first = dir.join("2026-02-01.md");
        let second = dir.join("2026-02-02.md");
        fs::write(&first, "Entry body\n").expect("first fixture file should be written");
        fs::write(&second, "Entry body\n").expect("second fixture file should be written");

        update_journal_headers(&dir).expect("updating headers should succeed");

        let first_contents = fs::read_to_string(&first).expect("first file should be readable");
        let second_contents = fs::read_to_string(&second).expect("second file should be readable");

        let first_header = extract_between_dashes(&first_contents)
            .and_then(parse_header)
            .expect("first file should have parseable header");
        let second_header = extract_between_dashes(&second_contents)
            .and_then(parse_header)
            .expect("second file should have parseable header");

        assert_eq!(first_header.prev, "");
        assert_eq!(second_header.prev, "[[2026-02-01]]");

        fs::remove_dir_all(&dir).expect("fixture directory should be cleaned up");
    }

    #[test]
    fn update_journal_headers_handles_date_gaps_for_prev() {
        let dir = unique_temp_path("update_journal_headers_prev_gap");
        fs::create_dir_all(&dir).expect("fixture directory should be created");

        let first = dir.join("2026-02-01.md");
        let second = dir.join("2026-02-09.md");
        fs::write(&first, "Entry body\n").expect("first fixture file should be written");
        fs::write(&second, "Entry body\n").expect("second fixture file should be written");

        update_journal_headers(&dir).expect("updating headers should succeed");

        let second_contents = fs::read_to_string(&second).expect("second file should be readable");

        let second_header = extract_between_dashes(&second_contents)
            .and_then(parse_header)
            .expect("second file should have parseable header");

        assert_eq!(second_header.prev, "[[2026-02-01]]");

        fs::remove_dir_all(&dir).expect("fixture directory should be cleaned up");
    }

    #[test]
    fn update_journal_headers_omits_journal_field_from_header() {
        let dir = unique_temp_path("update_journal_headers_no_journal_field");
        fs::create_dir_all(&dir).expect("fixture directory should be created");

        let file = dir.join("2026-02-01.md");
        fs::write(&file, "Entry body\n").expect("fixture file should be written");

        update_journal_headers(&dir).expect("updating headers should succeed");

        let contents = fs::read_to_string(&file).expect("file should be readable");

        let header = extract_between_dashes(&contents)
            .and_then(parse_header)
            .expect("file should have parseable header");

        assert_eq!(header.journal, None);
        assert!(!contents.contains("journal:"));

        fs::remove_dir_all(&dir).expect("fixture directory should be cleaned up");
    }

    #[test]
    fn update_journal_headers_uses_wiki_link_format() {
        let dir = unique_temp_path("update_journal_headers_wiki_link");
        fs::create_dir_all(&dir).expect("fixture directory should be created");

        let first = dir.join("2026-02-01.md");
        let second = dir.join("2026-02-02.md");
        fs::write(&first, "Entry body\n").expect("first fixture file should be written");
        fs::write(&second, "Entry body\n").expect("second fixture file should be written");

        update_journal_headers(&dir).expect("updating headers should succeed");

        let second_contents = fs::read_to_string(&second).expect("second file should be readable");

        let second_header = extract_between_dashes(&second_contents)
            .and_then(parse_header)
            .expect("second file should have parseable header");

        assert_eq!(second_header.prev, "[[2026-02-01]]");

        fs::remove_dir_all(&dir).expect("fixture directory should be cleaned up");
    }

    #[test]
    fn add_journal_header_skips_missing_files_silently() {
        let dir = unique_temp_path("add_journal_header_missing_file");
        fs::create_dir_all(&dir).expect("fixture directory should be created");

        let missing_file = dir.join("2026-02-01.md");

        // Should not error, just skip
        super::add_journal_header(&missing_file, "[[2026-01-31]]")
            .expect("should not error for missing file");

        assert!(!missing_file.exists());

        fs::remove_dir_all(&dir).expect("fixture directory should be cleaned up");
    }

    #[test]
    fn update_journal_headers_sets_first_file_prev_to_empty_string() {
        let dir = unique_temp_path("update_journal_headers_first_file_empty_prev");
        fs::create_dir_all(&dir).expect("fixture directory should be created");

        let first = dir.join("2026-02-01.md");
        fs::write(&first, "Entry body\n").expect("fixture file should be written");

        update_journal_headers(&dir).expect("updating headers should succeed");

        let first_contents = fs::read_to_string(&first).expect("first file should be readable");

        let first_header = extract_between_dashes(&first_contents)
            .and_then(parse_header)
            .expect("first file should have parseable header");

        assert_eq!(first_header.prev, "");

        fs::remove_dir_all(&dir).expect("fixture directory should be cleaned up");
    }
}
