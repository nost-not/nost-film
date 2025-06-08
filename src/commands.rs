use crate::files::get_pathes;
use crate::files::{append, get_last_not_path};
use chrono::Local;
use chrono::NaiveTime;
use std::path::PathBuf;
use std::{fs, io};
use uuid::Uuid;

pub fn print_commands() {
    eprintln!("Available commands:");
    eprintln!("  view                       Get a list of previous film viewings");
    eprintln!("  view <title>               Append a visionnage reference in last not");
    eprintln!("  view <title> <hh:mm>       Append a visionnage reference with specified time");
}

pub fn list_film_viewings(not_path: PathBuf) -> io::Result<()> {
    println!("Listing film viewings...");

    for current_path in get_pathes(not_path).unwrap() {
        let content = fs::read_to_string(current_path)?;
        let lines: Vec<&str> = content.lines().collect();

        for (i, line) in lines.iter().enumerate() {
            if line.contains("[nost-film] Visionnage") {
                for next_line in lines.iter().skip(i + 4).take(1) {
                    println!("{}", next_line);
                }
            }
        }
    }

    Ok(())
}

pub fn append_film_viewing(
    not_path: PathBuf,
    title: &str,
    viewing_time: Option<&str>,
) -> io::Result<()> {
    println!("Appending film viewing...");
    // let last_not_path = get_last_not_path(not_path)?;
    let last_not_path = match get_last_not_path(not_path) {
        Ok(path) => path,
        Err(err) => {
            eprintln!("Error in get_last_not_path: {}", err);
            return Err(err);
        }
    };

    let now = Local::now();

    // Check and validate the viewing_time
    let checked_time = viewing_time
        .and_then(|time| {
            NaiveTime::parse_from_str(time, "%H:%M")
                .map(|_| time.to_string())
                .ok()
        })
        .unwrap_or_else(|| now.format("%H:%M").to_string());

    let uid = Uuid::new_v4();
    let content = format!(
        "\n## [nost-film] Visionnage\n\n[//]: # \"not_film:{{uid: {}, time: {}, name: {} }}\"\n\n{} - {}",
        uid, checked_time, title, title, checked_time
    );

    append(last_not_path, &content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;
    use std::io::Write;

    #[test]
    fn test_run_append_film_viewing_with_time() {
        let temp_dir: tempfile::TempDir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("00001.md");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "Film viewings").unwrap();

        // Call run_append_film_viewing to append a film viewing
        let result = append_film_viewing(
            temp_dir.path().to_str().unwrap().into(),
            "Inception",
            Some("20:00"),
        );
        assert!(result.is_ok());

        // Verify that the film viewing was appended
        let mut file_content = String::new();
        File::open(&file_path)
            .unwrap()
            .read_to_string(&mut file_content)
            .unwrap();

        assert!(file_content.contains("Inception - 20:00"));
    }

    #[test]
    fn test_run_append_film_viewing_without_time() {
        let temp_dir: tempfile::TempDir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("00001.md");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "Film viewings").unwrap();

        let current_time = chrono::Local::now().format("%H:%M").to_string();

        // Call run_append_film_viewing to append a film viewing
        let result = append_film_viewing(
            temp_dir.path().to_str().unwrap().into(),
            "The Prestige",
            None,
        );
        assert!(result.is_ok());

        // Verify that the film viewing was appended
        let mut file_content = String::new();
        File::open(&file_path)
            .unwrap()
            .read_to_string(&mut file_content)
            .unwrap();

        assert!(file_content.contains(&format!("The Prestige - {}", current_time)));
    }

    #[test]
    fn test_run_append_film_viewing_with_invalid_time() {
        let temp_dir: tempfile::TempDir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("00001.md");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "Film viewings").unwrap();

        let current_time = chrono::Local::now().format("%H:%M").to_string();

        // Call run_append_film_viewing to append a film viewing
        let result = append_film_viewing(
            temp_dir.path().to_str().unwrap().into(),
            "Memento",
            Some("invalid_time"),
        );
        assert!(result.is_ok());

        let mut file_content = String::new();
        File::open(&file_path)
            .unwrap()
            .read_to_string(&mut file_content)
            .unwrap();

        // invalid time should be replaced with current time
        assert!(file_content.contains(&format!("Memento - {}", current_time)));
    }
}
