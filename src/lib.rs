pub struct Header {
    pub prev: String,
    pub journal: Option<String>,
}

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

impl std::fmt::Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "---\nprev: {}", self.prev)?;
        if let Some(journal) = &self.journal {
            write!(f, "\njournal: {}", journal)?;
        }
        write!(f, "\n---")
    }
}

pub fn extract_between_dashes(input: &str) -> Option<&str> {
    let start = input.find("---")?;
    let rest = &input[start + 3..];
    let end = rest.find("---")?;
    Some(&rest[..end])
}

pub fn parse_header(header: &str) -> Option<Header> {
    let mut prev: Option<String> = None;
    let mut journal: Option<String> = None;

    for line in header.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let (key, value) = trimmed.split_once(':')?;
        let key = key.trim();
        let value = value.trim().to_string();

        match key {
            "prev" => {
                if prev.is_some() {
                    return None;
                }
                prev = Some(value);
            }
            "journal" => {
                if journal.is_some() {
                    return None;
                }
                journal = Some(value);
            }
            _ => {}
        }
    }

    Some(Header {
        prev: prev?,
        journal,
    })
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
    fn extracts_inner_text_with_trailing_data() {
        let input = "prefix ---wanted section--- trailing data that should be ignored";
        assert_eq!(extract_between_dashes(input), Some("wanted section"));
    }

    #[test]
    fn returns_none_when_opening_delimiter_missing() {
        let input = "prefix data only";
        assert_eq!(extract_between_dashes(input), None);
    }

    #[test]
    fn returns_none_when_closing_delimiter_missing() {
        let input = "prefix ---starts but never ends";
        assert_eq!(extract_between_dashes(input), None);
    }

    #[test]
    fn extracts_empty_section() {
        let input = "------";
        assert_eq!(extract_between_dashes(input), Some(""));
    }

    #[test]
    fn uses_first_complete_delimiter_pair() {
        let input = "aaa ---first--- bbb ---second--- ccc";
        assert_eq!(extract_between_dashes(input), Some("first"));
    }

    #[test]
    fn header_to_string_formats_yaml_frontmatter() {
        let header = Header {
            prev: String::from("yesterday-note"),
            journal: Some(String::from("daily-log")),
        };

        assert_eq!(
            header.to_string(),
            "---\nprev: yesterday-note\njournal: daily-log\n---"
        );
    }

    #[test]
    fn parse_header_reads_prev_and_journal() {
        let parsed = parse_header("prev: yesterday-note\njournal: daily-log");

        assert!(parsed.is_some());
        let header = parsed.unwrap();
        assert_eq!(header.prev, "yesterday-note");
        assert_eq!(header.journal, Some(String::from("daily-log")));
    }

    #[test]
    fn parse_header_ignores_unknown_keys() {
        let parsed = parse_header("prev: yesterday-note\nfoo: bar\njournal: daily-log");

        assert!(parsed.is_some());
        let header = parsed.unwrap();
        assert_eq!(header.prev, "yesterday-note");
        assert_eq!(header.journal, Some(String::from("daily-log")));
    }

    #[test]
    fn parse_header_returns_none_when_prev_missing() {
        assert!(parse_header("journal: daily-log").is_none());
    }

    #[test]
    fn parse_header_allows_missing_optional_journal() {
        let parsed = parse_header("prev: yesterday-note");

        assert!(parsed.is_some());
        let header = parsed.unwrap();
        assert_eq!(header.prev, "yesterday-note");
        assert_eq!(header.journal, None);
    }

    #[test]
    fn parse_header_returns_none_for_malformed_non_empty_line() {
        assert!(parse_header("prev: yesterday-note\nnot-a-pair\njournal: daily-log").is_none());
    }

    #[test]
    fn parse_header_returns_none_for_duplicate_required_keys() {
        assert!(parse_header("prev: one\nprev: two\njournal: daily-log").is_none());
        assert!(parse_header("prev: yesterday-note\njournal: one\njournal: two").is_none());
    }

    #[test]
    fn parse_header_trims_whitespace() {
        let parsed = parse_header("  prev : yesterday-note  \n  journal: daily-log   ");

        assert!(parsed.is_some());
        let header = parsed.unwrap();
        assert_eq!(header.prev, "yesterday-note");
        assert_eq!(header.journal, Some(String::from("daily-log")));
    }

    #[test]
    fn extract_and_parse_header_flow() {
        let input =
            "prefix ---\nprev: yesterday-note\nextra: value\njournal: daily-log\n--- trailing";
        let parsed = extract_between_dashes(input).and_then(parse_header);

        assert!(parsed.is_some());
        let header = parsed.unwrap();
        assert_eq!(header.prev, "yesterday-note");
        assert_eq!(header.journal, Some(String::from("daily-log")));
    }

    #[test]
    fn header_to_string_omits_journal_when_missing() {
        let header = Header {
            prev: String::from("yesterday-note"),
            journal: None,
        };

        assert_eq!(header.to_string(), "---\nprev: yesterday-note\n---");
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
