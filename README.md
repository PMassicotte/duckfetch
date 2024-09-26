# duckfetch: a DuckDB manager for Linux

A simple (**very experimental**) Rust application to manage DuckDB installations, allowing users to list available versions and install specific versions easily.

## Demo

![Demo](./assets/demo.gif)

## Features

- List available DuckDB versions from GitHub releases.
- Install a specific DuckDB version.
- Automatically download and extract the selected version.

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) (version 1.60 or later)
- Internet access to download DuckDB releases.

## Installation

Clone the repository and install using Cargo:

```bash
cargo install --git https://github.com/pmassicotte/duckfetch
```

## Usage

### Check the currently installed DuckDB version

```bash
duckfetch check
```

### List available DuckDB versions

```bash
duckfetch list
```

### Install DuckDB

This command will print the available versions and ask the user to select one to install.

```bash
duckfetch install
```

## Help

For more information, use the `--help` flag:

```bash
duckfetch install --help
```
