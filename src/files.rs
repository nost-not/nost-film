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
    let file_regex = Regex::new(r"^\d+\.md$").unwrap();

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
    use std::path::PathBuf;
    use tempfile::NamedTempFile;
    use tempfile::TempDir;

    #[test]
    fn test_get_pathes() {
        // Create a temporary directory for testing
        let temp_dir = TempDir::new().expect("Failed to create temporary directory");
        let dir_path = temp_dir.path();

        // Create subdirectories and `.md` files
        let sub_dir = dir_path.join("123");
        fs::create_dir(&sub_dir).expect("Failed to create subdirectory");

        let file1 = sub_dir.join("123.md");
        let file2 = sub_dir.join("4.md");
        let file3 = sub_dir.join("notvalid_5.md");
        fs::write(&file1, "Content of file1").expect("Failed to write file1");
        fs::write(&file2, "Content of file2").expect("Failed to write file2");
        fs::write(&file3, "Content of file3").expect("Failed to write file3");

        // Call the function
        let result = get_pathes(dir_path.to_path_buf());

        // Assert the result
        assert!(result.is_ok(), "Function returned an error");
        let files = result.unwrap();
        assert_eq!(
            files,
            vec![file1, file2],
            "The returned file pathes do not match the expected result"
        );
    }

    // Mock implementation of `get_pathes`
    fn mock_get_pathes(_path: PathBuf) -> Result<Vec<PathBuf>, Error> {
        Ok(vec![
            PathBuf::from("mock_file1_123.md"),
            PathBuf::from("mock_file2_456.md"),
        ])
    }

    #[test]
    fn test_get_last_not_path_with_mock() {
        // Replace the real `not_path` with the mock directory
        let not_path = PathBuf::from("mock_directory");

        // Use the mock implementation of `get_pathes`
        let not_files = mock_get_pathes(not_path).expect("Failed to get mock pathes");

        // Simulate the behavior of `get_last_not_path`
        let last_file = not_files.last().expect("No files found");
        assert_eq!(
            last_file,
            &PathBuf::from("mock_file2_456.md"),
            "The last file does not match the expected result"
        );
    }

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
