use io_test::{extract_between_dashes, list_mds, parse_header, Header};
use serde::Deserialize;
use std::env;
use std::fs;
use std::io::{self, ErrorKind};
use std::path::{Path, PathBuf};

const CONFIG_RELATIVE_PATH: &str = ".config/chronicler/config.toml";

#[derive(Debug, Deserialize)]
struct Config {
    journal_directory: String,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(1);
    }
}

fn run() -> std::io::Result<()> {
    let journal_directory = load_journal_directory()?;

    if should_update_headers(std::env::args()) {
        update_headers(&journal_directory)?;
    }

    Ok(())
}

fn load_journal_directory() -> io::Result<PathBuf> {
    let config_path = chronicler_config_path()?;
    let config = load_config_from_path(&config_path)?;
    let journal_directory = config.journal_directory.trim();

    if journal_directory.is_empty() {
        return Err(io::Error::new(
            ErrorKind::InvalidData,
            format!(
                "`journal_directory` cannot be empty in {}",
                config_path.display()
            ),
        ));
    }

    Ok(PathBuf::from(journal_directory))
}

fn chronicler_config_path() -> io::Result<PathBuf> {
    let home = env::var_os("HOME").ok_or_else(|| {
        io::Error::new(
            ErrorKind::NotFound,
            "Cannot resolve HOME for chronicler config",
        )
    })?;

    Ok(PathBuf::from(home).join(CONFIG_RELATIVE_PATH))
}

fn load_config_from_path(config_path: &Path) -> io::Result<Config> {
    let contents = fs::read_to_string(config_path).map_err(|err| {
        if err.kind() == ErrorKind::NotFound {
            io::Error::new(
                ErrorKind::NotFound,
                format!("Missing chronicler config at {}", config_path.display()),
            )
        } else {
            err
        }
    })?;

    parse_config(&contents, config_path)
}

fn parse_config(contents: &str, config_path: &Path) -> io::Result<Config> {
    if contents.trim().is_empty() {
        return Err(io::Error::new(
            ErrorKind::InvalidData,
            format!(
                "Chronicler config at {} is empty. Expected TOML like: journal_directory = \"/path/to/journal\"",
                config_path.display()
            ),
        ));
    }

    toml::from_str::<Config>(contents).map_err(|err| {
        io::Error::new(
            ErrorKind::InvalidData,
            format!(
                "Invalid chronicler config at {}. Expected TOML like: journal_directory = \"/path/to/journal\". Parser details: {err}",
                config_path.display()
            ),
        )
    })
}

fn should_update_headers(args: impl IntoIterator<Item = String>) -> bool {
    args.into_iter()
        .skip(1)
        .any(|arg| arg == "--update-headers" || arg == "-u")
}

fn update_headers(journal_directory: impl AsRef<Path>) -> io::Result<()> {
    let md_files = list_mds(journal_directory)?;
    let dated_md_files: Vec<PathBuf> = md_files
        .into_iter()
        .filter(|path| is_dated_markdown_filename(path))
        .collect();

    if dated_md_files.is_empty() {
        return Err(io::Error::new(
            ErrorKind::NotFound,
            "No YYYY-MM-DD.md files found",
        ));
    }

    Ok(
        for (index, file_path) in dated_md_files.iter().enumerate() {
            let prev = if index == 0 {
                String::new()
            } else {
                let previous_date = dated_md_files[index - 1]
                    .file_stem()
                    .and_then(|stem| stem.to_str())
                    .ok_or_else(|| {
                        io::Error::new(
                            ErrorKind::InvalidData,
                            format!(
                                "Dated markdown filename is not valid UTF-8: {}",
                                dated_md_files[index - 1].display()
                            ),
                        )
                    })?
                    .to_string();

                format!("[[chronicle-{previous_date}]]")
            };

            println!("Processing: {}", file_path.display());
            add_header(file_path, &prev)?;
        },
    )
}

fn is_dated_markdown_filename(path: &Path) -> bool {
    let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
        return false;
    };

    if name.len() != 13 {
        return false;
    }

    let bytes = name.as_bytes();
    bytes[0..4].iter().all(u8::is_ascii_digit)
        && bytes[4] == b'-'
        && bytes[5..7].iter().all(u8::is_ascii_digit)
        && bytes[7] == b'-'
        && bytes[8..10].iter().all(u8::is_ascii_digit)
        && bytes[10] == b'.'
        && bytes[11] == b'm'
        && bytes[12] == b'd'
}

fn add_header(file_path: impl AsRef<std::path::Path>, prev: &str) -> Result<(), std::io::Error> {
    let file_path = file_path.as_ref();
    let contents = match fs::read_to_string(file_path) {
        Ok(contents) => contents,
        Err(err) if err.kind() == ErrorKind::NotFound => {
            println!("File does not exist: {}", file_path.display());
            return Ok(());
        }
        Err(err) => return Err(err),
    };
    let has_valid_header = extract_between_dashes(&contents)
        .and_then(parse_header)
        .is_some();
    if has_valid_header {
        println!("Valid header already exists. No changes made.");
        return Ok(());
    }

    let current_date = file_path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .ok_or_else(|| {
            io::Error::new(
                ErrorKind::InvalidData,
                format!(
                    "Dated markdown filename is not valid UTF-8: {}",
                    file_path.display()
                ),
            )
        })?;

    let header = Header {
        prev: prev.to_string(),
        journal: Some(format!("[[{current_date}]]")),
    };
    let updated_contents = format!("{}\n{}", header, contents);
    fs::write(file_path, updated_contents)?;

    println!("Header added to file.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        is_dated_markdown_filename, load_config_from_path, parse_config, should_update_headers,
        update_headers,
    };
    use io_test::{extract_between_dashes, parse_header};
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
    fn does_not_update_headers_without_flag() {
        let args = vec![String::from("io-test")];
        assert!(!should_update_headers(args));
    }

    #[test]
    fn updates_headers_with_long_flag() {
        let args = vec![String::from("io-test"), String::from("--update-headers")];
        assert!(should_update_headers(args));
    }

    #[test]
    fn updates_headers_with_short_flag() {
        let args = vec![String::from("io-test"), String::from("-u")];
        assert!(should_update_headers(args));
    }

    #[test]
    fn parse_config_reads_journal_directory() {
        let config = parse_config(
            "journal_directory = \"/tmp/journal\"",
            Path::new("/tmp/chronicler-config"),
        )
        .expect("config should parse");

        assert_eq!(config.journal_directory, "/tmp/journal");
    }

    #[test]
    fn parse_config_returns_invalid_data_for_missing_journal_directory() {
        let err = parse_config(
            "something_else = \"value\"",
            Path::new("/tmp/chronicler-config"),
        )
        .expect_err("missing key should fail");

        assert_eq!(err.kind(), ErrorKind::InvalidData);
    }

    #[test]
    fn parse_config_returns_invalid_data_for_malformed_toml() {
        let err = parse_config(
            "journal_directory = /tmp/journal",
            Path::new("/tmp/chronicler-config"),
        )
        .expect_err("malformed toml should fail");

        assert_eq!(err.kind(), ErrorKind::InvalidData);
    }

    #[test]
    fn parse_config_returns_invalid_data_for_empty_file() {
        let err = parse_config("\n\n", Path::new("/tmp/chronicler-config"))
            .expect_err("empty config should fail");

        assert_eq!(err.kind(), ErrorKind::InvalidData);
        assert!(err
            .to_string()
            .contains("Expected TOML like: journal_directory = \"/path/to/journal\""));
    }

    #[test]
    fn load_config_returns_not_found_for_missing_file() {
        let path = unique_temp_path("missing_chronicler_config");
        let err = load_config_from_path(&path).expect_err("missing config should fail");

        assert_eq!(err.kind(), ErrorKind::NotFound);
    }

    #[test]
    fn dated_markdown_filename_matches_expected_pattern() {
        assert!(is_dated_markdown_filename(Path::new("2026-01-31.md")));
    }

    #[test]
    fn dated_markdown_filename_rejects_non_matching_names() {
        let non_matches = [
            "notes.md",
            "2026-1-31.md",
            "2026-01-31.MD",
            "2026-01-31.txt",
            "20260131.md",
            "2026-01-31.md.bak",
        ];

        for file_name in non_matches {
            assert!(
                !is_dated_markdown_filename(Path::new(file_name)),
                "{file_name} should not match YYYY-MM-DD.md"
            );
        }
    }

    #[test]
    fn update_headers_only_updates_dated_markdown_files() {
        let dir = unique_temp_path("update_headers_dated_only");
        fs::create_dir_all(&dir).expect("fixture directory should be created");

        let dated_file = dir.join("2026-01-01.md");
        let non_dated_md = dir.join("notes.md");
        let txt_file = dir.join("2026-01-02.txt");

        fs::write(&dated_file, "Entry body\n").expect("dated fixture file should be written");
        fs::write(&non_dated_md, "Entry body\n")
            .expect("non-dated md fixture file should be written");
        fs::write(&txt_file, "Entry body\n").expect("txt fixture file should be written");

        update_headers(&dir).expect("updating headers should succeed");

        let dated_contents =
            fs::read_to_string(&dated_file).expect("dated file should be readable");
        let non_dated_contents =
            fs::read_to_string(&non_dated_md).expect("non-dated md should be readable");
        let txt_contents = fs::read_to_string(&txt_file).expect("txt file should be readable");

        assert!(extract_between_dashes(&dated_contents).is_some());
        assert_eq!(non_dated_contents, "Entry body\n");
        assert_eq!(txt_contents, "Entry body\n");

        fs::remove_dir_all(&dir).expect("fixture directory should be cleaned up");
    }

    #[test]
    fn update_headers_returns_not_found_when_no_dated_markdown_files() {
        let dir = unique_temp_path("update_headers_no_dated_files");
        fs::create_dir_all(&dir).expect("fixture directory should be created");

        fs::write(dir.join("notes.md"), "Entry body\n")
            .expect("non-dated md fixture file should be written");

        let err = update_headers(&dir).expect_err("should fail when no dated markdown files exist");
        assert_eq!(err.kind(), ErrorKind::NotFound);

        fs::remove_dir_all(&dir).expect("fixture directory should be cleaned up");
    }

    #[test]
    fn update_headers_sets_prev_to_previous_dated_file() {
        let dir = unique_temp_path("update_headers_prev_previous");
        fs::create_dir_all(&dir).expect("fixture directory should be created");

        let first = dir.join("2026-02-01.md");
        let second = dir.join("2026-02-02.md");
        fs::write(&first, "Entry body\n").expect("first fixture file should be written");
        fs::write(&second, "Entry body\n").expect("second fixture file should be written");

        update_headers(&dir).expect("updating headers should succeed");

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
    fn update_headers_handles_date_gaps_for_prev() {
        let dir = unique_temp_path("update_headers_prev_gap");
        fs::create_dir_all(&dir).expect("fixture directory should be created");

        let first = dir.join("2026-02-01.md");
        let second = dir.join("2026-02-09.md");
        fs::write(&first, "Entry body\n").expect("first fixture file should be written");
        fs::write(&second, "Entry body\n").expect("second fixture file should be written");

        update_headers(&dir).expect("updating headers should succeed");

        let second_contents = fs::read_to_string(&second).expect("second file should be readable");

        let second_header = extract_between_dashes(&second_contents)
            .and_then(parse_header)
            .expect("second file should have parseable header");

        assert_eq!(second_header.prev, "[[chronicle-2026-02-01]]");

        fs::remove_dir_all(&dir).expect("fixture directory should be cleaned up");
    }
}
