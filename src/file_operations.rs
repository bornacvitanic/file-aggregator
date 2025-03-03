use std::fs;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

const PATH_LINE_IDENTIFIER: &str = "=== FILE => ";
const DELETED_FILE_IDENTIFIER: &str = "=== ERASE => ";

#[derive(Debug)]
enum FileAction {
    Write(PathBuf, String),
    Delete(PathBuf),
}

pub(crate) fn get_file_paths(
    root_path: &PathBuf,
    whitelisted_file_types: &[String],
) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for entry in WalkDir::new(root_path).into_iter().filter_map(|e| e.ok()) {
        if is_valid_file(entry.path(), whitelisted_file_types) {
            let file_path = entry.path().to_path_buf();
            println!("Reading {}", &file_path.display());
            files.push(file_path);
        }
    }
    files
}

fn is_valid_file(path: &Path, whitelisted_file_types: &[String]) -> bool {
    match path.metadata() {
        Ok(metadata) if metadata.is_file() => {
            if whitelisted_file_types.is_empty() {
                return true;
            }
            if let Some(extension) = path.extension().and_then(|e| e.to_str()) {
                return whitelisted_file_types
                    .iter()
                    .any(|ext| ext.eq_ignore_ascii_case(extension));
            }
            false
        }
        _ => false,
    }
}

pub fn distribute_contents(root_path: &Path, clipboard_text: &str) -> io::Result<()> {
    let file_actions = parse_combined_contents(clipboard_text);
    for file_action in file_actions {
        match file_action {
            FileAction::Write(relative_path, content) => {
                let file_path = root_path.join(relative_path);
                println!("Writing {}", &file_path.display());
                write_file(file_path, &content)?;
            }
            FileAction::Delete(relative_path) => {
                let file_path = root_path.join(relative_path);
                println!("Deleting {}", &file_path.display());
                delete_file(file_path)?;
            }
        }
    }
    Ok(())
}

fn write_file(file_path: PathBuf, content: &str) -> io::Result<()> {
    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut file = File::create(file_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn delete_file(file_path: PathBuf) -> io::Result<()> {
    if file_path.exists() {
        fs::remove_file(file_path)?;
    } else {
        println!("File not found: {}", &file_path.display());
    }
    Ok(())
}

fn make_relative(absolute_path: &Path, base_path: &Path) -> Option<PathBuf> {
    if absolute_path.starts_with(base_path) {
        let relative_path = absolute_path.strip_prefix(base_path).ok()?;
        Some(relative_path.to_path_buf())
    } else {
        None
    }
}

pub fn combine_file_contents(root_path: &Path, file_paths: &[PathBuf]) -> io::Result<String> {
    let mut combined_result = String::new();
    for file_path in file_paths.iter() {
        if let Some(relative_path) = make_relative(file_path, root_path) {
            let mut contents = String::new();
            let mut file = File::open(file_path)?;
            file.read_to_string(&mut contents)?;
            combined_result.push_str(&format!(
                "{}{}\n",
                PATH_LINE_IDENTIFIER,
                relative_path.display()
            ));
            combined_result.push_str(&contents);
            combined_result.push('\n');
        }
    }
    Ok(combined_result)
}

fn parse_combined_contents(clipboard_text: &str) -> Vec<FileAction> {
    let mut files_contents = Vec::new();
    let mut lines = clipboard_text.lines().peekable();
    while let Some(line) = lines.next() {
        if line.starts_with(PATH_LINE_IDENTIFIER) {
            let relative_path = line.trim_start_matches(PATH_LINE_IDENTIFIER).trim();
            let mut content = String::new();
            while let Some(content_line) = lines.peek() {
                if content_line.starts_with(PATH_LINE_IDENTIFIER)
                    || content_line.starts_with(DELETED_FILE_IDENTIFIER)
                {
                    break;
                }
                content.push_str(content_line);
                content.push('\n');
                lines.next(); // Move to the next line
            }
            files_contents.push(FileAction::Write(PathBuf::from(relative_path), content));
        } else if line.starts_with(DELETED_FILE_IDENTIFIER) {
            let relative_path = line.trim_start_matches(DELETED_FILE_IDENTIFIER).trim();
            files_contents.push(FileAction::Delete(PathBuf::from(relative_path)));
        }
    }
    files_contents
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_is_valid_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        fs::write(&file_path, "content").unwrap();

        let whitelist = vec!["txt".to_string()];
        assert!(is_valid_file(&file_path, &whitelist));

        let empty_whitelist: Vec<String> = vec![];
        assert!(is_valid_file(&file_path, &empty_whitelist));

        let invalid_whitelist = vec!["md".to_string()];
        assert!(!is_valid_file(&file_path, &invalid_whitelist));
    }

    #[test]
    fn test_is_valid_file_no_extension() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("file");
        fs::write(&file_path, "content").unwrap();

        let whitelist = vec!["txt".to_string()];
        assert!(!is_valid_file(&file_path, &whitelist));
    }

    #[test]
    fn test_is_valid_file_hidden_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join(".hiddenfile.txt");
        fs::write(&file_path, "content").unwrap();

        let whitelist = vec!["txt".to_string()];
        assert!(is_valid_file(&file_path, &whitelist));
    }

    #[test]
    fn test_is_valid_file_case_insensitive_extension() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("file.TXT");
        fs::write(&file_path, "content").unwrap();

        let whitelist = vec!["txt".to_string()];
        assert!(is_valid_file(&file_path, &whitelist));
    }

    #[test]
    fn test_is_valid_file_empty_whitelist() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("file.txt");
        fs::write(&file_path, "content").unwrap();

        let empty_whitelist: Vec<String> = vec![];
        assert!(is_valid_file(&file_path, &empty_whitelist));
    }

    #[test]
    fn test_is_valid_file_empty_path() {
        let path = PathBuf::new();
        let whitelist = vec!["txt".to_string()];
        assert!(!is_valid_file(&path, &whitelist));
    }

    #[test]
    fn test_is_valid_file_invalid_metadata() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("nonexistentfile.txt");

        let whitelist = vec!["txt".to_string()];
        assert!(!is_valid_file(&file_path, &whitelist));
    }

    #[test]
    fn test_is_valid_file_directory_instead_of_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        let dir_path = temp_dir.path().join("subdir");
        fs::create_dir(&dir_path).unwrap();

        let whitelist = vec!["txt".to_string()];
        assert!(!is_valid_file(&dir_path, &whitelist));
    }

    #[test]
    fn test_get_file_paths() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path_1 = temp_dir.path().join("test1.txt");
        let file_path_2 = temp_dir.path().join("test2.md");
        fs::write(&file_path_1, "content1").unwrap();
        fs::write(&file_path_2, "content2").unwrap();

        let whitelist = vec!["txt".to_string()];
        let result = get_file_paths(&temp_dir.path().to_path_buf(), &whitelist);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], file_path_1);
    }

    #[test]
    fn test_write_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        write_file(file_path.clone(), "content").unwrap();

        let mut content = String::new();
        let mut file = File::open(file_path).unwrap();
        file.read_to_string(&mut content).unwrap();
        assert_eq!(content, "content");
    }

    #[test]
    fn test_distribute_contents() {
        let temp_dir = tempfile::tempdir().unwrap();
        let root_path = temp_dir.path().to_path_buf();
        let clipboard_text = format!(
            "{0}test1.txt\ncontent1\n{0}test2.md\ncontent2\n",
            PATH_LINE_IDENTIFIER
        );
        distribute_contents(&root_path, &*clipboard_text).unwrap();

        let file_path_1 = root_path.join("test1.txt");
        let file_path_2 = root_path.join("test2.md");

        assert!(file_path_1.exists());
        assert!(file_path_2.exists());

        let mut content = String::new();
        let mut file = File::open(file_path_1).unwrap();
        file.read_to_string(&mut content).unwrap();
        assert_eq!(content, "content1\n");

        content.clear();
        let mut file = File::open(file_path_2).unwrap();
        file.read_to_string(&mut content).unwrap();
        assert_eq!(content, "content2\n");
    }

    #[test]
    fn test_delete_file() {
        // Create a temporary directory
        let temp_dir = tempfile::tempdir().unwrap();
        let root_path = temp_dir.path().to_path_buf();

        // Create a temporary file to be deleted
        let file_path = root_path.join("delete_me.txt");
        fs::write(&file_path, "temporary content").unwrap();

        // Ensure the file exists before deletion
        assert!(file_path.exists());

        // Prepare clipboard text to delete the file
        let clipboard_text = format!("{0}delete_me.txt\n", DELETED_FILE_IDENTIFIER);

        // Distribute contents to delete the file
        distribute_contents(&root_path, &clipboard_text).unwrap();

        // Ensure the file does not exist after deletion
        assert!(!file_path.exists());
    }

    #[test]
    fn test_make_relative() {
        let base_path = Path::new("/base");
        let absolute_path = Path::new("/base/dir/file.txt");
        let relative_path = make_relative(absolute_path, base_path).unwrap();
        assert_eq!(relative_path, PathBuf::from("dir/file.txt"));

        let invalid_path = Path::new("/other/file.txt");
        assert!(make_relative(invalid_path, base_path).is_none());
    }

    #[test]
    fn test_combine_file_contents() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path_1 = temp_dir.path().join("test1.txt");
        let file_path_2 = temp_dir.path().join("test2.txt");
        fs::write(&file_path_1, "content1").unwrap();
        fs::write(&file_path_2, "content2").unwrap();

        let root_path = temp_dir.path().to_path_buf();
        let file_paths = vec![file_path_1, file_path_2];
        let combined = combine_file_contents(&root_path, &file_paths).unwrap();

        let expected = format!(
            "{0}test1.txt\ncontent1\n{0}test2.txt\ncontent2\n",
            PATH_LINE_IDENTIFIER
        );
        assert_eq!(combined, expected);
    }

    #[test]
    fn test_parse_combined_contents() {
        let clipboard_text = format!(
            "{0}test1.txt\ncontent1\n{0}test2.txt\ncontent2\n{1}test3.txt\n",
            PATH_LINE_IDENTIFIER, DELETED_FILE_IDENTIFIER
        );
        let parsed = parse_combined_contents(&clipboard_text);

        assert_eq!(parsed.len(), 3);

        match &parsed[0] {
            FileAction::Write(path, content) => {
                assert_eq!(path, &PathBuf::from("test1.txt"));
                assert_eq!(content, "content1\n");
            }
            _ => panic!("Expected FileAction::Write"),
        }

        match &parsed[1] {
            FileAction::Write(path, content) => {
                assert_eq!(path, &PathBuf::from("test2.txt"));
                assert_eq!(content, "content2\n");
            }
            _ => panic!("Expected FileAction::Write"),
        }

        match &parsed[2] {
            FileAction::Delete(path) => {
                assert_eq!(path, &PathBuf::from("test3.txt"));
            }
            _ => panic!("Expected FileAction::Delete"),
        }
    }
}
