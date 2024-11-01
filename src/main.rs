//! This module provides the main entry point for the DuckDB CLI application.
//!
//! The application supports the following subcommands:
//!
//! - `list`: Lists the available DuckDB versions.
//! - `check`: Checks the current setup.
//! - `install`: Prompts the user to select a DuckDB version to install and installs it.
//! - `update`: Install the latest version of DuckDB.
//!
//! The main function builds the command-line interface (CLI) using `build_cli` and
//! matches the subcommands provided by the user to perform the corresponding actions.

mod duckfetch;

use duckfetch::artifacts::ArtifactsResponse;
use duckfetch::build_cli;
use duckfetch::check;
use duckfetch::completion::generate_completions;
use duckfetch::duckdb_versions;
use duckfetch::install_duckdb;

use anyhow::{Context, Result};
use duckfetch::version::latest_stable_release;

fn main() -> Result<()> {
    // Not ready yet for Windows
    if cfg!(target_os = "windows") {
        eprintln!("This CLI is not ready yet for Windows. Exiting.");
        std::process::exit(1);
    }

    let mut app = build_cli();
    let matches = app.get_matches_mut();

    match matches.subcommand() {
        Some(("list", _)) => {
            duckdb_versions()?.print_versions();
        }
        Some(("check", _)) => {
            check()?;
        }
        Some(("install", _)) => {
            let available_versions = duckdb_versions()?;
            let tag_names = available_versions.releases();
            let selected_tag = inquire::Select::new(
                "Select the DuckDB version to install (Esc to cancel): ",
                tag_names,
            )
            .prompt()
            .context("Error")?;

            let release = available_versions
                .release_by_tag(&selected_tag)
                .context("Can not find the specified tag version")?;

            install_duckdb(release)?;
        }
        Some(("update", _)) => {
            let latest_version = latest_stable_release()?;

            let available_versions = duckdb_versions()?;

            let release = available_versions
                .release_by_tag(&latest_version)
                .context("err")?;

            install_duckdb(release)?;
        }
        Some(("changelog", _)) => {
            webbrowser::open("https://github.com/duckdb/duckdb/releases")
                .context("Could not open the release web page")?;
        }
        Some(("completions", _)) => {
            generate_completions();
        }
        _ => {
            app.print_help()?;
            println!();
        }
    }

    Ok(())
}
