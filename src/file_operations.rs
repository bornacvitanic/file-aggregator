use std::fs;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub(crate) fn get_file_paths(root_path: &PathBuf, whitelisted_file_types: &Vec<String>) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for entry in WalkDir::new(root_path).into_iter().filter_map(|e| e.ok()) {
        if is_valid_file(&entry.path(), &whitelisted_file_types) {
            files.push(entry.path().to_path_buf());
        }
    }
    files
}

fn is_valid_file(path: &Path, whitelisted_file_types: &Vec<String>) -> bool {
    match path.metadata() {
        Ok(metadata) if metadata.is_file() => {
            if whitelisted_file_types.is_empty() {
                return true;
            }
            if let Some(extension) = path.extension().and_then(|e| e.to_str()) {
                return whitelisted_file_types.iter().any(|ext| ext.eq_ignore_ascii_case(extension));
            }
            false
        }
        _ => false,
    }
}

pub fn distribute_contents(root_path: &PathBuf, clipboard_text: &str) -> io::Result<()> {
    let files_contents = parse_combined_contents(clipboard_text);
    for (relative_path, content) in files_contents {
        write_file(root_path.join(relative_path), &content)?;
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

fn make_relative(absolute_path: &Path, base_path: &Path) -> Option<PathBuf> {
    if absolute_path.starts_with(base_path) {
        let relative_path = absolute_path.strip_prefix(base_path).ok()?;
        Some(relative_path.to_path_buf())
    } else {
        None
    }
}

const PATH_LINE_IDENTIFIER: &'static str = "===";

pub fn combine_file_contents(root_path: &PathBuf, file_paths: &Vec<PathBuf>) -> io::Result<String> {
    let mut combined_result = String::new();
    for file_path in file_paths.iter() {
        if let Some(relative_path) = make_relative(file_path, root_path) {
            let mut contents = String::new();
            let mut file = File::open(file_path)?;
            file.read_to_string(&mut contents)?;
            combined_result.push_str(&format!("{}{}\n", PATH_LINE_IDENTIFIER, relative_path.display()));
            combined_result.push_str(&contents);
            combined_result.push('\n');
        }
    }
    Ok(combined_result)
}

fn parse_combined_contents(clipboard_text: &str) -> Vec<(PathBuf, String)> {
    let mut files_contents = Vec::new();
    let mut lines = clipboard_text.lines().peekable();
    while let Some(line) = lines.next() {
        if line.starts_with(PATH_LINE_IDENTIFIER) {
            let relative_path = line.trim_start_matches(PATH_LINE_IDENTIFIER);
            let mut content = String::new();
            while let Some(content_line) = lines.peek() {
                if content_line.starts_with(PATH_LINE_IDENTIFIER) {
                    break;
                }
                content.push_str(content_line);
                content.push('\n');
                lines.next(); // Move to the next line
            }
            files_contents.push((PathBuf::from(relative_path.trim()), content));
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
        let clipboard_text = format!("{0}test1.txt\ncontent1\n{0}test2.md\ncontent2\n", PATH_LINE_IDENTIFIER);
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

        let expected = format!("{0}test1.txt\ncontent1\n{0}test2.txt\ncontent2\n", PATH_LINE_IDENTIFIER);
        assert_eq!(combined, expected);
    }

    #[test]
    fn test_parse_combined_contents() {
        let clipboard_text = format!("{0}test1.txt\ncontent1\n{0}test2.txt\ncontent2\n", PATH_LINE_IDENTIFIER);
        let parsed = parse_combined_contents(&*clipboard_text);

        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].0, PathBuf::from("test1.txt"));
        assert_eq!(parsed[0].1, "content1\n");
        assert_eq!(parsed[1].0, PathBuf::from("test2.txt"));
        assert_eq!(parsed[1].1, "content2\n");
    }
}