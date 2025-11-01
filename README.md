# txtcv CLI

txtcv is a modern, minimal CLI program for managing your txtcv CV from the terminal. It
helps you spin up a new CV, validate it against the JSON resume schema, authenticate
with [txtcv.com], and publish updates in seconds.

## Features
- `init` scaffolds a starter `cv.json` file you can customise.
- `validate` checks your CV against the bundled JSON Schema before you publish.
- `auth` manages your personal access token (login, logout, check).
- `publish` uploads the contents of `cv.json` to txtcv.com using your token.

## Installation

The CLI can be installed either using Homebrew or using Cargo.

### Homebrew installation

```sh
brew install txtcv/tap/txtcv
```

### Cargo installation

```sh
# build and install locally
cargo install --path .

# alternatively, build the binary without installing
cargo build --release
```

## Usage
Display the built-in help:
```sh
txtcv --help
```

Typical workflow:
```sh
# 1. Create a starter cv.json
 txtcv init

# 2. Fill in cv.json with your details

# 3. Validate before publishing
 txtcv validate

# 4. Authenticate with your personal access token
 txtcv auth login

# 5. Publish updates to txtcv.com
 txtcv publish --cv-id <your-cv-id>
```

### Authentication details
`txtcv auth login` securely stores your personal access token using the
[`confy`](https://docs.rs/confy/latest/confy/) crate. Subsequent commands reuse the
stored token until you run `txtcv auth logout` to clear it.

## Links
- Homepage: https://txtcv.com/cli/
- Repository: https://github.com/txtcv/cli

[txtcv.com]: https://txtcv.com
