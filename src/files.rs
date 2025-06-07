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

pub fn append_content(file_path: PathBuf, content: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&file_path)?;

    writeln!(file, "{}", content)?;
    println!("Content appended successfully to {}", file_path.display());
    Ok(())
}
