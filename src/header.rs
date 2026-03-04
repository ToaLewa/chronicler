//! YAML frontmatter header parsing and formatting for chronicle files.
//!
//! Chronicle files use YAML frontmatter headers with `prev` and optional `journal` fields.

/// Represents a chronicle file's YAML frontmatter header.
pub struct Header {
    pub prev: String,
    pub journal: Option<String>,
}

impl std::fmt::Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "---\nprev: \"{}\"", self.prev)?;
        if let Some(journal) = &self.journal {
            write!(f, "\njournal: \"{}\"", journal)?;
        }
        write!(f, "\n---")
    }
}

/// Extracts content between the first pair of `---` delimiters.
///
/// Returns `None` if the input doesn't contain both opening and closing delimiters.
pub fn extract_between_dashes(input: &str) -> Option<&str> {
    let start = input.find("---")?;
    let rest = &input[start + 3..];
    let end = rest.find("---")?;
    Some(&rest[..end])
}

/// Parses a YAML header string into a `Header` struct.
///
/// Returns `None` if the header is malformed or missing required fields.
/// The `prev` field is required, while `journal` is optional.
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
        let value = value.trim();
        let value = value
            .strip_prefix('"')
            .and_then(|inner| inner.strip_suffix('"'))
            .unwrap_or(value)
            .to_string();

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
            "---\nprev: \"yesterday-note\"\njournal: \"daily-log\"\n---"
        );
    }

    #[test]
    fn parse_header_reads_prev_and_journal() {
        let parsed = parse_header("prev: \"yesterday-note\"\njournal: \"daily-log\"");

        assert!(parsed.is_some());
        let header = parsed.unwrap();
        assert_eq!(header.prev, "yesterday-note");
        assert_eq!(header.journal, Some(String::from("daily-log")));
    }

    #[test]
    fn parse_header_ignores_unknown_keys() {
        let parsed = parse_header("prev: \"yesterday-note\"\nfoo: bar\njournal: \"daily-log\"");

        assert!(parsed.is_some());
        let header = parsed.unwrap();
        assert_eq!(header.prev, "yesterday-note");
        assert_eq!(header.journal, Some(String::from("daily-log")));
    }

    #[test]
    fn parse_header_returns_none_when_prev_missing() {
        assert!(parse_header("journal: \"daily-log\"").is_none());
    }

    #[test]
    fn parse_header_allows_missing_optional_journal() {
        let parsed = parse_header("prev: \"yesterday-note\"");

        assert!(parsed.is_some());
        let header = parsed.unwrap();
        assert_eq!(header.prev, "yesterday-note");
        assert_eq!(header.journal, None);
    }

    #[test]
    fn parse_header_returns_none_for_malformed_non_empty_line() {
        assert!(
            parse_header("prev: \"yesterday-note\"\nnot-a-pair\njournal: \"daily-log\"").is_none()
        );
    }

    #[test]
    fn parse_header_returns_none_for_duplicate_required_keys() {
        assert!(parse_header("prev: \"one\"\nprev: \"two\"\njournal: \"daily-log\"").is_none());
        assert!(
            parse_header("prev: \"yesterday-note\"\njournal: \"one\"\njournal: \"two\"").is_none()
        );
    }

    #[test]
    fn parse_header_trims_whitespace() {
        let parsed = parse_header("  prev : \"yesterday-note\"  \n  journal: \"daily-log\"   ");

        assert!(parsed.is_some());
        let header = parsed.unwrap();
        assert_eq!(header.prev, "yesterday-note");
        assert_eq!(header.journal, Some(String::from("daily-log")));
    }

    #[test]
    fn extract_and_parse_header_flow() {
        let input =
            "prefix ---\nprev: \"yesterday-note\"\nextra: value\njournal: \"daily-log\"\n--- trailing";
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

        assert_eq!(header.to_string(), "---\nprev: \"yesterday-note\"\n---");
    }
}
