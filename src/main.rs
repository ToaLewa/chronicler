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
    Ok(for file_path in md_files {
        println!("Processing: {}", file_path.display());
        add_header(&file_path)?;
    })
}

fn add_header(file_path: impl AsRef<std::path::Path>) -> Result<(), std::io::Error> {
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
    let hardcoded_header = Header {
        prev: String::from("[[yesterday-note]]"),
        journal: Some(String::from("daily-log")),
    };
    let updated_contents = format!("{}\n{}", hardcoded_header, contents);
    fs::write(file_path, updated_contents)?;

    println!("Header added to file.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{load_config_from_path, parse_config, should_update_headers};
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
}
