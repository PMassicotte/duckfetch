# duckup: a DuckDB manager for Linux

A simple Rust application to manage DuckDB installations, allowing users to list available versions and install specific versions easily.

## Features

- List available DuckDB versions from GitHub releases.
- Install a specific DuckDB version.
- Automatically download and extract the selected version.

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) (version 1.60 or later)
- Internet access to download DuckDB releases.

## Installation

Clone the repository:

```bash
cargo install --git https://github.com/pmassicotte/duckup
```

## Usage

List available DuckDB versions:

```bash
duckup list
```

Install a specific DuckDB version:

```bash
duckup install 0.4.0
```

Instll the latest DuckDB version:

```bash
duckup install
```
