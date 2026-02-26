use io_test::{extract_between_dashes, list_mds, parse_header, Header};
use std::fs;
use std::io::ErrorKind;

fn main() -> std::io::Result<()> {
    let journal_directory = "./tests";
    if should_update_headers(std::env::args()) {
        update_headers(journal_directory)?;
    } 

    Ok(())
}

fn should_update_headers(args: impl IntoIterator<Item = String>) -> bool {
    args.into_iter()
        .skip(1)
        .any(|arg| arg == "--update-headers" || arg == "-u")
}

fn update_headers(journal_directory: &str) -> Result<(), std::io::Error> {
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
    use super::should_update_headers;

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
}
