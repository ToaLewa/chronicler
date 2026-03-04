//! Configuration file loading and parsing.
//!
//! Loads chronicler settings from `~/.config/chronicler/config.toml`.

use serde::Deserialize;
use std::env;
use std::fs;
use std::io::{self, ErrorKind};
use std::path::{Path, PathBuf};

const CONFIG_RELATIVE_PATH: &str = ".config/chronicler/config.toml";

/// Configuration loaded from the TOML config file.
#[derive(Debug, Deserialize)]
pub struct Config {
    pub chronicler_directory: Option<String>,
}

/// Loads the chronicler directory path from the config file.
///
/// Returns an error if the config file is missing, malformed, or the directory path is empty.
pub fn load_chronicler_directory() -> io::Result<PathBuf> {
    let config_path = chronicler_config_path()?;
    let config = load_config_from_path(&config_path)?;
    let chronicler_directory = config.chronicler_directory.as_deref().unwrap_or("").trim();

    if chronicler_directory.is_empty() {
        return Err(io::Error::new(
            ErrorKind::InvalidData,
            format!(
                "`chronicler_directory` cannot be empty in {}",
                config_path.display()
            ),
        ));
    }

    Ok(PathBuf::from(chronicler_directory))
}

/// Returns the expected path to the chronicler config file.
///
/// The config file is located at `~/.config/chronicler/config.toml`.
pub fn chronicler_config_path() -> io::Result<PathBuf> {
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
                "Chronicler config at {} is empty. Expected TOML like: chronicler_directory = \"/path/to/chronicler\"",
                config_path.display()
            ),
        ));
    }

    toml::from_str::<Config>(contents).map_err(|err| {
        io::Error::new(
            ErrorKind::InvalidData,
            format!(
                "Invalid chronicler config at {}. Expected TOML like: chronicler_directory = \"/path/to/chronicler\". Parser details: {err}",
                config_path.display()
            ),
        )
    })
}

#[cfg(test)]
mod tests {
    use super::{load_config_from_path, parse_config};
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
    fn parse_config_reads_chronicler_directory() {
        let config = parse_config(
            "chronicler_directory = \"/tmp/chronicler\"",
            Path::new("/tmp/chronicler-config"),
        )
        .expect("config should parse");

        assert_eq!(
            config.chronicler_directory.as_deref(),
            Some("/tmp/chronicler")
        );
    }

    #[test]
    fn parse_config_reads_chronicler_directory_from_same_config_as_journal() {
        let config = parse_config(
            "journal_directory = \"/tmp/journal\"\nchronicler_directory = \"/tmp/chronicler\"",
            Path::new("/tmp/chronicler-config"),
        )
        .expect("config with both directories should parse");

        assert_eq!(
            config.chronicler_directory.as_deref(),
            Some("/tmp/chronicler")
        );
    }

    #[test]
    fn parse_config_allows_missing_chronicler_directory() {
        let err = parse_config(
            "something_else = \"value\"",
            Path::new("/tmp/chronicler-config"),
        );

        assert!(err.is_ok());
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
            .contains("Expected TOML like: chronicler_directory = \"/path/to/chronicler\""));
    }

    #[test]
    fn load_config_returns_not_found_for_missing_file() {
        let path = unique_temp_path("missing_chronicler_config");
        let err = load_config_from_path(&path).expect_err("missing config should fail");

        assert_eq!(err.kind(), ErrorKind::NotFound);
    }
}
