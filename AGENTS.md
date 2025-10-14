# Repository Guidelines

## Project Structure & Module Organization
- `src/main.rs` hosts the CLI entrypoint and the feature logic
- JSON assets (`src/alice.json`, `src/schema.json`) stay alongside the code so they can
  be embedded with `include_str!`.
- Add new functionality under `src/` (e.g., `src/auth.rs`) and expose it with `mod ...;`
  in `main.rs` or `lib.rs`.

## Build, Test, and Development Commands
- `cargo build` – compile the project and catch missing imports early.
- `cargo fmt` – format all sources; required before submitting changes.
- `cargo clippy` – run lint checks; treat warnings as failures.
- `cargo test` / `cargo test <name>` – execute the full suite or a focused case.
- `cargo run -- <args>` – exercise the CLI locally (e.g., `cargo run -- validate`).

## Coding Style & Naming Conventions
- Use rustfmt defaults (4-space indentation, trailing commas). Group imports: `std`,
  external crates, local modules.
- Functions/variables use `snake_case`; types use `PascalCase`; constants use
  `SCREAMING_SNAKE_CASE`.
- Prefer `Result`/`Option` over panics; log recoverable issues with `eprintln!` and
  success with `println!`.
- Document public APIs with `///` comments, especially when modules move into `lib.rs`.

## Security & Configuration Tips
- `confy` stores tokens locally; never commit real credentials or test data.
- Handle HTTP errors by returning informative messages and exit codes so the CLI behaves
  predictably.
- Sanitize sample CV data before publishing to protect personal information.
