use chronicler::{
    append_chronicle_entry, load_chronicler_directory, load_config, update_chronicler_headers,
    update_journal_headers,
};
use std::env;
use std::io::{self, ErrorKind};
use std::path::PathBuf;

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(1);
    }
}

fn run() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    // Check if we're in update-headers mode
    if should_update_headers(args.iter().cloned()) {
        // Load config once
        let config = load_config()?;

        let chronicler_directory = config
            .chronicler_directory
            .as_deref()
            .ok_or_else(|| {
                io::Error::new(
                    ErrorKind::InvalidData,
                    "`chronicler_directory` is missing from config",
                )
            })?
            .trim();

        if chronicler_directory.is_empty() {
            return Err(io::Error::new(
                ErrorKind::InvalidData,
                "`chronicler_directory` cannot be empty",
            ));
        }

        let journal_directory = config
            .journal_directory
            .as_deref()
            .ok_or_else(|| {
                io::Error::new(
                    ErrorKind::InvalidData,
                    "`journal_directory` is missing from config",
                )
            })?
            .trim();

        if journal_directory.is_empty() {
            return Err(io::Error::new(
                ErrorKind::InvalidData,
                "`journal_directory` cannot be empty",
            ));
        }

        let chronicler_path = PathBuf::from(chronicler_directory);
        let journal_path = PathBuf::from(journal_directory);

        println!(
            "Looking for chronicle files in {}\n",
            chronicler_path.display()
        );
        update_chronicler_headers(&chronicler_path)?;

        println!(
            "\nLooking for journal files in {}\n",
            journal_path.display()
        );
        update_journal_headers(&journal_path)?;

        return Ok(());
    }

    // For append mode, use the existing load function for backward compatibility
    let chronicler_directory = load_chronicler_directory()?;

    // Check if we're in append mode (positional argument provided)
    if args.len() >= 2 {
        let entry_text = &args[1];
        append_chronicle_entry(&chronicler_directory, entry_text)?;
        println!("Entry appended to today's chronicle.");
        return Ok(());
    }

    // No valid arguments provided - show usage
    eprintln!("Usage:");
    eprintln!("  chronicler \"entry text\"              Append an entry to today's chronicle");
    eprintln!("  chronicler --update-headers          Update headers for all chronicle files");
    eprintln!("  chronicler -u                        Update headers (short form)");

    std::process::exit(1);
}

fn should_update_headers(args: impl IntoIterator<Item = String>) -> bool {
    args.into_iter()
        .skip(1)
        .any(|arg| arg == "--update-headers" || arg == "-u")
}

#[cfg(test)]
mod tests {
    use super::should_update_headers;

    #[test]
    fn does_not_update_headers_without_flag() {
        let args = vec![String::from("chronicler")];
        assert!(!should_update_headers(args));
    }

    #[test]
    fn updates_headers_with_long_flag() {
        let args = vec![String::from("chronicler"), String::from("--update-headers")];
        assert!(should_update_headers(args));
    }

    #[test]
    fn updates_headers_with_short_flag() {
        let args = vec![String::from("chronicler"), String::from("-u")];
        assert!(should_update_headers(args));
    }
}
