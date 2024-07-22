use std::string::String;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use clap::{Parser};
use walkdir::WalkDir;
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


    if let Some(extensions) = options.extensions {
        println!("Using extensions: {:?}", extensions);
        // Extension logic here
    }

    walk_directory(&root_path);
}

fn walk_directory(root_path: &PathBuf) {
    for entry in WalkDir::new(&root_path).into_iter().filter_map(|e| e.ok()) {
        match make_relative(entry.path(), &root_path) {
            Some(relative_path) => {
                match File::open(entry.path()) {
                    Ok(mut file) => {
                        let mut contents = String::new();
                        if let Err(err) = file.read_to_string(&mut contents) {
                            eprintln!("Error reading file {}: {}", entry.path().display(), err);
                        } else {
                            println!("//{}", relative_path.display());
                            println!("{}\n", contents);
                        }
                    },
                    Err(err) => {
                        eprintln!("Error opening file {}: {}", entry.path().display(), err);
                    },
                }
            },
            None => println!("The base path is not an ancestor of the absolute path."),
        }
    }
}

fn distribute(options: Options) {
    println!("{}", "Distributing files...");

    let root_path = options.path.unwrap_or_else(|| std::env::current_dir().unwrap());

    println!("Using path: {:?}", root_path);


    if let Some(extensions) = options.extensions {
        println!("Using extensions: {:?}", extensions);
        // Extension logic here
    }

    walk_directory(&root_path);
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