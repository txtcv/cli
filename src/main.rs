use std::fs;
use std::path::Path;
use std::process;

use clap::{Parser, Subcommand};
use jsonschema;
use serde_json::Value;

/// txtcv is a modern and simple CV builder for folks in tech
#[derive(Debug, Parser)]
#[command(version, about, long_about=None, arg_required_else_help=true)]
struct CLI {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Initialize a CV in the current directory
    Init { filename: Option<String> },

    /// Validate the CV file in the current directory
    Validate { filename: Option<String> },
}

fn main() {
    let cli = CLI::parse();

    match cli.command {
        Some(Commands::Init { filename }) => {
            let filename = match filename {
                Some(name) => name,
                None => String::from("cv.json"),
            };
            let cv_json = serde_json::json!({
                "basics": {
                    "name": "Alice",
                    "email": "alice@example.com",
                }
            });
            let cv_data = serde_json::to_string_pretty(&cv_json);

            if Path::new(&filename).exists() {
                println!("{} already exists.", filename);
            } else {
                fs::write(&filename, cv_data.unwrap().as_bytes()).unwrap();
                println!("Created {}.", &filename);
            }
        }
        Some(Commands::Validate { filename }) => {
            let filename = match filename {
                Some(name) => name,
                None => String::from("cv.json"),
            };

            let path = Path::new(&filename);

            if !path.exists() {
                println!("{} does not exist.", filename);
                return;
            }

            let cv_data = fs::read_to_string(path).unwrap();
            let instance = serde_json::from_str::<Value>(&cv_data).unwrap();

            let json_resume_schema = include_str!("schema.json");
            let schema = serde_json::from_str::<Value>(&json_resume_schema).unwrap();

            match jsonschema::validate(&schema, &instance) {
                Ok(_) => {
                    println!("{} is valid.", filename);
                }
                Err(err) => {
                    eprintln!("Error: {err}");
                    process::exit(1);
                }
            }
        }
        None => (),
    };
}
