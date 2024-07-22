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