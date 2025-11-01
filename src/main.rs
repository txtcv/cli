use std::default::Default;
use std::env;
use std::fs;
use std::io;
use std::path::Path;
use std::process;

use clap::{Args, Parser, Subcommand};
use confy;
use jsonschema;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use ureq;

/// txtcv is a modern and simple CV builder for folks in tech
#[derive(Debug, Parser)]
#[command(version, about, long_about=None, arg_required_else_help=true)]
struct CLI {
    #[command(subcommand)]
    command: Option<Commands>,
}

/// Publish CV file to https://txtcv.com
#[derive(Debug, Args)]
#[command(arg_required_else_help = true)]
struct PublishArgs {
    #[arg(short, long)]
    cv_id: String,

    #[arg(short, long, default_value = "cv.json")]
    filename: String,
}

#[derive(Debug, Args)]
struct InitArgs {
    #[arg(short, long, default_value = "cv.json")]
    filename: String,
}

#[derive(Debug, Args)]
struct ValidateArgs {
    #[arg(short, long, default_value = "cv.json")]
    filename: String,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Initialize a CV file in the current directory
    Init(InitArgs),

    /// Validate the CV file in the current directory
    Validate(ValidateArgs),

    /// Authentication
    Auth(AuthArgs),

    /// Publish the CV file in the current directory
    Publish(PublishArgs),
}

/// Manage authentication with https://txtcv.com
#[derive(Debug, Args)]
#[command(arg_required_else_help = true)]
struct AuthArgs {
    #[command(subcommand)]
    command: Option<AuthCommands>,
}

#[derive(Debug, Subcommand)]
enum AuthCommands {
    /// Log in using your personal access token
    Login,

    /// Clear the currently stored personal access token
    Logout,

    /// Check whether the current personal access token is valid
    Check,
}

#[derive(Debug, Serialize, Deserialize)]
struct AppConfig {
    version: u8,
    personal_access_token: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            version: 0,
            personal_access_token: String::from(""),
        }
    }
}

fn main() {
    let cli = CLI::parse();

    let exit_code = match cli.command {
        Some(Commands::Init(init)) => run_init(init.filename),
        Some(Commands::Validate(validate)) => run_validate(validate.filename),
        Some(Commands::Auth(auth)) => {
            let auth_command = auth.command.unwrap();

            match auth_command {
                AuthCommands::Login => run_auth_login(),
                AuthCommands::Logout => run_auth_logout(),
                AuthCommands::Check => run_auth_check(),
            }
        }
        Some(Commands::Publish(publish)) => run_publish(publish.cv_id, publish.filename),
        None => 1,
    };

    if exit_code > 0 {
        process::exit(exit_code);
    }
}

fn run_init(filename: String) -> i32 {
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

fn run_validate(filename: String) -> i32 {
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

fn run_auth_login() -> i32 {
    let mut config: AppConfig = confy::load("txtcv", None).unwrap();
    let mut access_token = String::new();

    println!("Please enter your personal access token: ");
    io::stdin()
        .read_line(&mut access_token)
        .expect("Failed to read line");

    config.personal_access_token = String::from(access_token.trim());

    match confy::store("txtcv", None, config) {
        Ok(_) => {
            println!("Logged in!");
            return 0;
        }
        Err(err) => {
            eprintln!("Error: {err}.");
            eprintln!("Please try again.");
            return 1;
        }
    };
}

fn run_auth_logout() -> i32 {
    let mut config: AppConfig = confy::load("txtcv", None).unwrap();

    config.personal_access_token = String::new();

    match confy::store("txtcv", None, config) {
        Ok(_) => {
            println!("Logged out!");
            return 0;
        }
        Err(err) => {
            eprintln!("Error: {err}.");
            eprintln!("Please try again.");
            return 1;
        }
    };
}

fn run_auth_check() -> i32 {
    let token = match get_personal_access_token() {
        Some(value) => value,
        None => {
            eprintln!("Personal Access Token is missing.");
            eprintln!("Please run auth login and try again");
            return 1;
        }
    };

    let response = ureq::get("https://txtcv.com")
        .header("Authorization", format!("Bearer {token}", token = token))
        .call();

    match response {
        Ok(_) => {
            println!("It worked!");
            return 0;
        }
        Err(err) => {
            eprintln!("Something went wrong. Please run auth login and try again.");
            eprintln!("{:?}", err);
            return 1;
        }
    }
}

#[derive(Serialize)]
struct PatchRequest {
    contents: Value,
}

fn run_publish(cv_id: String, filename: String) -> i32 {
    let path = Path::new(&filename);

    if !path.exists() {
        eprintln!("{} does not exist", filename);
        return 1;
    }

    let cv_raw = fs::read_to_string(path).unwrap();
    let cv_json = serde_json::from_str::<Value>(&cv_raw).unwrap();

    let schema_raw = include_str!("schema.json");
    let schema_json = serde_json::from_str::<Value>(&schema_raw).unwrap();

    match jsonschema::validate(&schema_json, &cv_json) {
        Ok(_) => {
            let token = match get_personal_access_token() {
                Some(value) => value,
                None => {
                    eprintln!("Personal Access Token is missing.");
                    eprintln!("Please run auth login and try again");
                    return 1;
                }
            };

            let body = PatchRequest { contents: cv_json };
            let response = ureq::patch(format!("https://txtcv.com/api/cv/{cv_id}"))
                .header("Authorization", format!("Bearer {token}", token = token))
                .send_json(&body);

            match response {
                Ok(_) => {
                    println!("The CV contents have been updated.");
                    return 0;
                }
                Err(err) => {
                    eprintln!("Something went wrong. Please run auth login and try again.");
                    eprintln!("{:?}", err);
                    return 1;
                }
            };
        }
        Err(err) => {
            eprintln!("Error: {err}");
            return 1;
        }
    };
}

/// Returns the personal access token by checking the `TXTCV_AUTH_TOKEN` environment
/// variable first, then falling back to the persisted config if no override exists.
fn get_personal_access_token() -> Option<String> {
    let key = "TXTCV_AUTH_TOKEN";

    match env::var(key) {
        Ok(value) => {
            return Some(value);
        }
        Err(_) => {
            let config: AppConfig = confy::load("txtcv", None).unwrap();

            if config.personal_access_token.trim().is_empty() {
                return None;
            }

            return Some(config.personal_access_token);
        }
    }
}
