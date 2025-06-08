use crate::io::Error;
use regex::Regex;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::{fs, io};

pub fn get_pathes(path: PathBuf) -> Result<Vec<PathBuf>, Error> {
    let mut files = Vec::new();
    let mut pathes: Vec<PathBuf> = vec![path];

    let folder_regex = Regex::new(r"^\d+$").unwrap();
    let file_regex = Regex::new(r".*\d+\.md$").unwrap();

    while let Some(current) = pathes.pop() {
        match fs::read_dir(&current) {
            Ok(entries) => {
                for entry in entries.flatten() {
                    let current_path = entry.path();
                    if let Some(name) = current_path.file_name().and_then(|name| name.to_str()) {
                        if current_path.is_dir() {
                            if folder_regex.is_match(name) {
                                pathes.push(current_path);
                            }
                        } else if file_regex.is_match(name) {
                            files.push(current_path);
                        }
                    }
                }
            }
            Err(err) => return Err(err),
        }
    }

    files.sort();

    Ok(files)
}

pub fn get_last_not_path(not_path: PathBuf) -> io::Result<PathBuf> {
    let not_files =
        get_pathes(not_path.into()).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

    if let Some(last_file) = not_files.last() {
        Ok(last_file.to_path_buf())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "No `.md` files found in the specified directory",
        ))
    }
}

pub fn append(file_path: PathBuf, content: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&file_path)?;

    writeln!(file, "{}", content)?;
    println!("Content appended successfully to {}", file_path.display());
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;
    use std::io::Read;
    use tempfile::NamedTempFile;

    #[test]
    fn test_append() {
        // Create a temporary file for testing
        let temp_file = NamedTempFile::new().expect("Failed to create temporary file");
        let test_file_path = temp_file.path().to_path_buf();
        let initial_content = "Initial content.\n";
        let append_content = "Appended content.";

        // Write initial content to the file
        fs::write(&test_file_path, initial_content).expect("Failed to write initial content");

        // Append content using the function
        append(test_file_path.clone(), append_content).expect("Failed to append content");

        // Read the file to verify the content
        let mut file = fs::File::open(&test_file_path).expect("Failed to open test file");
        let mut file_content = String::new();
        file.read_to_string(&mut file_content)
            .expect("Failed to read test file");

        // Assert the content
        assert_eq!(
            file_content,
            format!("{}{}\n", initial_content, append_content),
            "File content does not match expected content"
        );
    }
}
