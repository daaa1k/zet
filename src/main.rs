use chrono::Local;
use clap::Parser;
use std::env;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

/// Zettelkasten-style note utility
#[derive(Parser, Debug)]
#[command(name = "zet")]
#[command(author = "Your Name")]
#[command(version = "1.0")]
#[command(about = "Create and open zettel notes", long_about = None)]
struct Args {
    filename: Option<String>,
}

fn get_filename() -> String {
    print!("Enter a filename: ");
    io::stdout().flush().unwrap();
    let mut filename = String::new();
    io::stdin().read_line(&mut filename).unwrap();
    filename.trim().to_string()
}

fn open_file(dir: &str, filename: &str) -> std::path::PathBuf {
    let file_path = format!("{}/{}.md", dir, filename);
    let path = Path::new(&file_path);

    fs::create_dir_all(dir).unwrap();

    if !path.exists() {
        fs::write(&path, "").unwrap();
    }

    let timestamp = Local::now().format("%Y%m%d%H%M").to_string();
    let content = format!("# \n\n\n\nLinks:\n\n{}", timestamp);

    let mut file = OpenOptions::new().append(true).open(&path).unwrap();
    writeln!(file, "{}", content).unwrap();

    #[cfg(not(test))]
    {
        let editor = env::var("EDITOR").expect("EDITOR env variable not set");
        Command::new(editor)
            .arg(&file_path)
            .status()
            .expect("failed to open editor");
    }

    path.to_path_buf()
}

fn zet(filename_opt: Option<String>) {
    let filename = match filename_opt {
        Some(f) => f,
        None => get_filename(),
    };

    if filename.contains(' ') {
        eprintln!("Please provide only one filename separated by dashes, without .md extension.");
        eprintln!("Example: zet my-new-note");
        return;
    }

    let base_dir = env::var("ZETTELKASTEN").expect("ZETTELKASTEN env variable not set");
    let zettel_dir = format!("{}/0_inbox", base_dir);

    open_file(&zettel_dir, &filename);
}

fn main() {
    let args = Args::parse();
    zet(args.filename);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Read;
    use tempfile::tempdir;

    #[test]
    fn test_open_file_creates_file_with_template() {
        let dir = tempdir().unwrap();
        let file_path = open_file(dir.path().to_str().unwrap(), "test-note");

        assert!(file_path.exists());

        let mut content = String::new();
        fs::File::open(&file_path)
            .unwrap()
            .read_to_string(&mut content)
            .unwrap();

        assert!(content.contains("Links:"));
    }

    #[test]
    fn test_filename_parsing() {
        let args = Args::parse_from(&["zet", "my-file"]);
        assert_eq!(args.filename.unwrap(), "my-file");
    }

    #[test]
    fn test_no_filename_prompts() {
        let args = Args::parse_from(&["zet"]);
        assert!(args.filename.is_none());
    }
}
