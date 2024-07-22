use std::string::String;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use clap::{Parser};
use walkdir::WalkDir;
use copypasta::{ClipboardContext, ClipboardProvider};
use crate::cli::{AppCommand, Cli, Options};

mod cli;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(AppCommand::Aggregate { options }) => {
            aggregate(options);
        }
        Some(AppCommand::Distribute { options }) => {
            distribute(options);
        }
        None => println!("No command provided"),
    }
}

fn aggregate(options: Options) {
    println!("{}", "Aggregating files...");

    let root_path = options.path.unwrap_or_else(|| std::env::current_dir().unwrap());

    println!("Using path: {:?}", root_path);

    let extensions = options.extensions.unwrap_or_default();

    let file_paths = get_file_paths(&root_path, &extensions);

    let contents = combine_file_contents(&root_path, &file_paths);
    match contents {
        None => { eprintln!("No content found.") }
        Some(contents) => {
            let mut ctx = ClipboardContext::new().unwrap();
            ctx.set_contents(contents).unwrap();
            println!("Copied contents to clipboard!")
        }
    }
}

fn combine_file_contents(root_path: &PathBuf, file_paths: &Vec<PathBuf>) -> Option<String> {
    let mut combined_result = String::new();
    for file_path in file_paths.iter() {
        match make_relative(file_path, &root_path) {
            Some(relative_path) => {
                let mut contents = String::new();
                match File::open(file_path) {
                    Ok(mut file) => {
                        if let Err(err) = file.read_to_string(&mut contents) {
                            eprintln!("Error reading file {}: {}", file_path.display(), err);
                        } else {
                            println!("Reading {}", &relative_path.display());
                            combined_result.push_str(&format!("==={}\n", relative_path.display()));
                            combined_result.push_str(&contents);
                            combined_result.push('\n');
                        }
                    }
                    Err(err) => {
                        eprintln!("Error opening file {}: {}", file_path.display(), err);
                    }
                }
            }
            None => println!("The base path is not an ancestor of the absolute path."),
        }
    }
    if combined_result.is_empty() {
        None
    } else {
        Some(combined_result)
    }
}

fn get_file_paths(root_path: &PathBuf, whitelisted_file_types: &Vec<String>) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for entry in WalkDir::new(&root_path).into_iter().filter_map(|e| e.ok()) {
        if entry.metadata().map(|m| {
            m.is_file() && (whitelisted_file_types.is_empty() || whitelisted_file_types.iter().any(|ext| {
                entry.path().extension().and_then(|e| e.to_str()).map(|e| e.eq_ignore_ascii_case(ext)).unwrap_or(false)
            }))
        }).unwrap_or(false) {
            files.push(entry.path().to_path_buf());
        }
    }
    files
}

fn distribute(options: Options) {
    println!("Distributing files...");

    let root_path = options.path.unwrap_or_else(|| std::env::current_dir().unwrap());

    println!("Using path: {:?}", root_path);

    let mut ctx = ClipboardContext::new().unwrap();
    let clipboard_text = ctx.get_contents().unwrap();

    let files_contents = parse_combined_contents(&clipboard_text);
    for (relative_path, content) in files_contents {
        let file_path = root_path.join(relative_path);
        if let Some(parent) = file_path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        println!("Writing {}", &file_path.display());
        std::fs::write(file_path, content).unwrap();
    }
}

fn parse_combined_contents(clipboard_text: &str) -> Vec<(PathBuf, String)> {
    let mut files_contents = Vec::new();
    let mut lines = clipboard_text.lines().peekable();
    while let Some(line) = lines.next() {
        if line.starts_with("===") {
            let relative_path = line.trim_start_matches("===");
            let mut content = String::new();
            while let Some(content_line) = lines.peek() {
                if content_line.starts_with("===") {
                    break;
                }
                content.push_str(content_line);
                content.push('\n');
                lines.next(); // Move to the next line
            }
            files_contents.push((PathBuf::from(relative_path), content));
        }
    }
    files_contents
}

fn make_relative(absolute_path: &Path, base_path: &Path) -> Option<PathBuf> {
    // Check if base_path is an ancestor of absolute_path
    if absolute_path.starts_with(base_path) {
        let relative_path = absolute_path.strip_prefix(base_path).ok()?;
        Some(relative_path.to_path_buf())
    } else {
        None
    }
}