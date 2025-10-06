use std::fs;
use std::path::Path;

use clap::{Parser, Subcommand};

/// txtcv is a modern and simple CV builder for folks in tech
#[derive(Debug, Parser)]
#[command(version, about, long_about=None, arg_required_else_help=true)]
struct CLI {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Initialize a cv.json in the current directory
    Init,
}

fn main() {
    let cli = CLI::parse();

    match cli.command {
        Some(Commands::Init) => {
            let filename = "cv.json";
            let cv_json = serde_json::json!({
                "basics": {
                    "name": "Alice",
                    "email": "alice@example.com",
                }
            });
            let cv_data = serde_json::to_string_pretty(&cv_json);

            if Path::new(filename).exists() {
                println!("{} already exists.", filename);
            } else {
                fs::write(filename, cv_data.unwrap().as_bytes()).unwrap();
                println!("Created {}.", filename);
            }
        }
        None => (),
    };
}
