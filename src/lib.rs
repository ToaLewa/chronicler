//! Chronicle file management library.
//!
//! This library provides functionality for managing chronicle files with YAML frontmatter headers.

pub mod chronicle;
pub mod config;
pub mod header;
pub mod journal;

pub use chronicle::{append_chronicle_entry, read_last_n_chronicles, update_chronicler_headers};
pub use config::{
    chronicler_config_path, load_chronicler_directory, load_config, load_journal_directory, Config,
};
pub use header::{extract_between_dashes, parse_header, Header};
pub use journal::update_journal_headers;

/// Lists all markdown files (.md) in a directory, sorted alphabetically.
///
/// This function is non-recursive and only returns files with lowercase `.md` extensions.
pub fn list_mds(path: impl AsRef<std::path::Path>) -> std::io::Result<Vec<std::path::PathBuf>> {
    let mut files = Vec::new();

    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();

        if entry_path.is_file() && entry_path.extension().and_then(|ext| ext.to_str()) == Some("md")
        {
            files.push(entry_path);
        }
    }

    files.sort();
    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_temp_path(prefix: &str) -> std::path::PathBuf {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be after UNIX_EPOCH")
            .as_nanos();

        std::env::temp_dir().join(format!("{prefix}_{}_{}", std::process::id(), stamp))
    }

    #[test]
    fn list_mds_returns_fixture_files() {
        let files = list_mds("tests/testing").expect("fixture directory should exist");
        let names: Vec<String> = files
            .iter()
            .map(|path| {
                path.file_name()
                    .and_then(|name| name.to_str())
                    .expect("fixture names should be valid UTF-8")
                    .to_string()
            })
            .collect();

        assert_eq!(names.len(), 5);
        assert_eq!(
            names,
            vec![
                "2026-01-01.md",
                "2026-01-02.md",
                "2026-01-03.md",
                "2026-01-04.md",
                "2026-01-05.md",
            ]
        );
        assert!(files.iter().all(|path| path.is_file()));
    }

    #[test]
    fn list_mds_is_not_recursive() {
        let base = unique_temp_path("io_test_non_recursive");
        let nested = base.join("nested");

        std::fs::create_dir_all(&nested).expect("should create nested fixture directory");
        std::fs::write(base.join("2026-02-01.md"), "").expect("should create root fixture file");
        std::fs::write(nested.join("2026-02-02.md"), "")
            .expect("should create nested fixture file");

        let files = list_mds(&base).expect("temp fixture directory should be readable");
        let names: Vec<String> = files
            .iter()
            .map(|path| {
                path.file_name()
                    .and_then(|name| name.to_str())
                    .expect("fixture names should be valid UTF-8")
                    .to_string()
            })
            .collect();

        assert_eq!(names, vec!["2026-02-01.md"]);

        std::fs::remove_dir_all(&base).expect("should clean up temp fixture directory");
    }

    #[test]
    fn list_mds_includes_only_lowercase_md_files() {
        let base = unique_temp_path("io_test_md_filter");

        std::fs::create_dir_all(&base).expect("should create temp fixture directory");
        std::fs::write(base.join("2026-03-01.md"), "").expect("should create md fixture file");
        std::fs::write(base.join("2026-03-02.txt"), "").expect("should create txt fixture file");
        std::fs::write(base.join("README"), "").expect("should create extensionless fixture file");
        std::fs::write(base.join("2026-03-03.MD"), "")
            .expect("should create uppercase extension file");

        let files = list_mds(&base).expect("temp fixture directory should be readable");
        let names: Vec<String> = files
            .iter()
            .map(|path| {
                path.file_name()
                    .and_then(|name| name.to_str())
                    .expect("fixture names should be valid UTF-8")
                    .to_string()
            })
            .collect();

        assert_eq!(names, vec!["2026-03-01.md"]);

        std::fs::remove_dir_all(&base).expect("should clean up temp fixture directory");
    }

    #[test]
    fn list_mds_returns_err_for_missing_directory() {
        let missing = unique_temp_path("io_test_missing_dir");
        assert!(list_mds(&missing).is_err());
    }
}
