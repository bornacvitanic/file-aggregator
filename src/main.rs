use clap::{Parser};
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
}

fn distribute(options: Options) {
    println!("{}", "Distributing files...");

    let root_path = options.path.unwrap_or_else(|| std::env::current_dir().unwrap());

    println!("Using path: {:?}", root_path);


    if let Some(extensions) = options.extensions {
        println!("Using extensions: {:?}", extensions);
        // Extension logic here
    }
}