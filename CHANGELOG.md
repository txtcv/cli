# Changelog

## 0.2.0

- Allow reading the personal access token from the `TXTCV_AUTH_TOKEN` environment
  variable before falling back to the config file

## 0.1.0

- Initial release
  - Add `init` and `validate` commands to initialize and validate a CV file in the
    current working directory
  - Add `publish` command to publish CV files to txtcv.com
  - Add `auth` command with `login`, `logout`, and `check` subcommands
