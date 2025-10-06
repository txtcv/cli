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
    /// Initialize a CV file in the current directory
    Init { filename: Option<String> },

    /// Validate the CV file in the current directory
    Validate { filename: Option<String> },
}

fn main() {
    let cli = CLI::parse();

    let exit_code = match cli.command {
        Some(Commands::Init { filename }) => run_init(filename),
        Some(Commands::Validate { filename }) => run_validate(filename),
        None => 1,
    };

    if exit_code > 0 {
        process::exit(exit_code);
    }
}

fn run_init(filename: Option<String>) -> i32 {
    let filename = match filename {
        Some(name) => name,
        None => String::from("cv.json"),
    };

    if Path::new(&filename).exists() {
        println!("{} already exists.", filename);
        return 1;
    }

    let cv_raw = include_str!("alice.json");
    let cv_json = serde_json::from_str::<Value>(cv_raw).unwrap();
    let cv_pretty = serde_json::to_string_pretty(&cv_json);

    fs::write(&filename, cv_pretty.unwrap().as_bytes()).unwrap();
    println!("Initialized {}.", &filename);
    return 0;
}

fn run_validate(filename: Option<String>) -> i32 {
    let filename = match filename {
        Some(name) => name,
        None => String::from("cv.json"),
    };

    let path = Path::new(&filename);

    if !path.exists() {
        println!("{} does not exist.", filename);
        return 1;
    }

    let cv_raw = fs::read_to_string(path).unwrap();
    let cv_json = serde_json::from_str::<Value>(&cv_raw).unwrap();

    let schema_raw = include_str!("schema.json");
    let schema_json = serde_json::from_str::<Value>(&schema_raw).unwrap();

    match jsonschema::validate(&schema_json, &cv_json) {
        Ok(_) => {
            println!("{} is valid.", filename);
            return 0;
        }
        Err(err) => {
            eprintln!("Error: {err}");
            return 1;
        }
    }
}
