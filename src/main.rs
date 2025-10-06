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
    let cv_json = serde_json::json!({
        "basics": {
            "name": "Alice",
            "label": "Staff Software Engineer | Open Source Maintainer",
            "email": "alice@example.com",
            "url": "https://alice.com",
            "summary": "Passionate and experienced software engineer with expertise in developing scalable web applications",
        },
        "profiles": [
            {
                "network": "LinkedIn",
                "username": "alice",
                "url": "https://www.linkedin.com/in/alice",
            }
        ],
        "work": [
            {
                "name": "Example Corp",
                "position": "Staff Software Engineer",
                "url": "https://example.com",
                "startDate": "2024-01-01",
                "summary": "Architected and led the development of a new cloud-native SaaS platform, setting technical direction and mentoring a team of 15 engineers.",
                "highlights": [
                    "Designed a microservices architecture that improved scalability by 200%.",
                    "Championed the adoption of CI/CD practices, reducing deployment time by 75%."
                ]
            },
        ],
        "education": [
            {
                "institution": "Example University",
                "url": "https://www.example.edu",
                "area": "Computer Science",
                "studyType": "Master's",
                "startDate": "2016-09-01",
                "endDate": "2018-06-01",
                "score": "3.9 GPA",
                "courses": ["Machine Learning", "Natural Language Processing"]
            }
        ],
        "skills": [
            {
                "name": "Programming Languages",
                "level": "Expert",
                "keywords": ["JavaScript", "Python", "Rust"]
            }
        ],
        "languages": [
          {"language": "English", "fluency": "Native speaker"}
        ],
        "interests": [
          {"name": "Open Source Contribution"},
          {"name": "Hiking"}
        ],
    });
    let cv_data = serde_json::to_string_pretty(&cv_json);

    if Path::new(&filename).exists() {
        println!("{} already exists.", filename);
        return 1;
    } else {
        fs::write(&filename, cv_data.unwrap().as_bytes()).unwrap();
        println!("Created {}.", &filename);
        return 0;
    }
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

    let cv_data = fs::read_to_string(path).unwrap();
    let instance = serde_json::from_str::<Value>(&cv_data).unwrap();

    let json_resume_schema = include_str!("schema.json");
    let schema = serde_json::from_str::<Value>(&json_resume_schema).unwrap();

    match jsonschema::validate(&schema, &instance) {
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
