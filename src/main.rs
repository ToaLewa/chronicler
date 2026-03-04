use chronicler::{append_chronicle_entry, load_chronicler_directory, update_chronicler_headers};
use std::env;

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(1);
    }
}

fn run() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let chronicler_directory = load_chronicler_directory()?;

    // Check if we're in update-headers mode
    if should_update_headers(args.iter().cloned()) {
        println!(
            "Looking for chrono files in {}\n",
            chronicler_directory.display()
        );
        update_chronicler_headers(&chronicler_directory)?;
        return Ok(());
    }

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
