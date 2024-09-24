# duckup: a DuckDB manager for Linux

<!-- [![Crates.io](https://img.shields.io/crates/v/duckup)](https://crates.io/crates/duckup) -->
<!-- [![License](https://img.shields.io/crates/l/duckup)]( -->

A simple (**very experimental**) Rust application to manage DuckDB installations, allowing users to list available versions and install specific versions easily.

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
cargo install --git https://github.com/pmassicotte/duckup
```

## Usage

### List available DuckDB versions

```bash
duckup list
```

### Install a specific DuckDB version

```bash
duckup install v0.4.0
```

### Install the latest DuckDB version

```bash
duckup install
```

## Help

For more information, use the `--help` flag:

```bash
duckup install --help
```
