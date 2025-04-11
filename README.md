# duckfetch: a DuckDB manager for Linux (maybe Windows and MacOS)

[![Crates.io](https://img.shields.io/crates/v/duckfetch)](https://crates.io/crates/duckfetch) [![License](https://img.shields.io/badge/license-Apache%202.0%20%7C%20MIT-blue.svg)](https://opensource.org/licenses/Apache-2.0) ![Build Status](https://github.com/PMassicotte/duckfetch/actions/workflows/release.yml/badge.svg) ![Build Status](https://github.com/PMassicotte/duckfetch/actions/workflows/rust.yml/badge.svg) [![Test Linux Installation](https://github.com/PMassicotte/duckfetch/actions/workflows/test-linux-install.yml/badge.svg)](https://github.com/PMassicotte/duckfetch/actions/workflows/test-linux-install.yml) [![Test macOS Installation](https://github.com/PMassicotte/duckfetch/actions/workflows/test-macos-install.yml/badge.svg)](https://github.com/PMassicotte/duckfetch/actions/workflows/test-macos-install.yml) [![Test Windows Installation](https://github.com/PMassicotte/duckfetch/actions/workflows/test-windos-install.yml/badge.svg)](https://github.com/PMassicotte/duckfetch/actions/workflows/test-windos-install.yml)

A simple Rust application to manage DuckDB installations, allowing users to list available versions and install specific versions easily.

## Table of Contents

- [Demo](#demo)
- [Features](#features)
- [Requirements](#requirements)
- [Installation](#installation)
- [Usage](#usage)
  - [Check the currently installed DuckDB version](#check-the-currently-installed-duckdb-version)
  - [List available DuckDB versions](#list-available-duckdb-versions)
  - [Install DuckDB](#install-duckdb)
- [Help](#help)
- [Autocompletion](#autocompletion)
- [Other related projects](#other-related-projects)

## Demo

![Demo](./assets/demo.gif)

## Features

- List available DuckDB versions from GitHub releases with `duckfetch list`.
- Install a specific DuckDB version with `duckfetch install`.
- Update to the latest version with `duckfetch update`.
- Generate auto-completion scripts for your shell with `duckfetch completions`.

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) (version 1.60 or later)
- Internet access to download DuckDB releases.

## Installation

To install it from crates.io:

```bash
cargo install duckfetch
```

Or the development version:

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

### Open the change log of the latest DuckDB version

This will open the GitHub release page for the latest DuckDB version in your default browser.

```bash
duckfetch changelog
```

## Help

For more information, use the `--help` flag:

```bash
duckfetch install --help
```

## Autocompletion

To enable autocompletion, you can use the `completions` subcommand to generate the completion script for your shell. For example, to enable completions for the `zsh` shell:

```bash
duckfetch completions zsh >~/.zfunc/_duckfetch
```

Then, add the following line to your `.zshrc` file:

```bash
fpath+=~/.zfunc
autoload -Uz compinit
compinit -i
```

## Other related projects

- https://github.com/NiclasHaderer/duckdb-version-manager
- https://github.com/carlopi/duckdb-latest
