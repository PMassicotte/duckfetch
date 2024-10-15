use clap::Arg;
use clap::ColorChoice;
use clap::{command, Command};
use clap_complete::Shell;

/// Builds the command-line interface (CLI) for the DuckDB utility.
///
/// This function sets up the CLI with the following subcommands:
/// - `list`: Lists all available DuckDB versions.
/// - `check`: Compares the installed version of DuckDB with the latest release.
/// - `install`: Installs a specific version of DuckDB given a proposed list of releases.
/// - `update`: Install the latest version of DuckDB.
///
/// # Returns
///
/// A `Command` object representing the CLI configuration.

pub fn build_cli() -> Command {
    command!()
        .subcommand(Command::new("list").about("Lists all available DuckDB versions"))
        .subcommand(
            Command::new("check")
                .about("Compare the installed version of DuckDB with the latest release"),
        )
        .subcommand(
            Command::new("install")
                .about("Installs a specific version of DuckDB given a proposed list of releases"),
        )
        .subcommand(Command::new("update").about("Update DuckDB to the latest version"))
        .subcommand(
            Command::new("changelog")
                .about("Open the release changelog in the default web browser"),
        )
        .subcommand(
            Command::new("completions")
                .about("Generate autocompletion for a specified shell")
                .arg(
                    Arg::new("shell")
                        .help("The shell to generate completions for")
                        .required(true)
                        .value_parser(clap::builder::EnumValueParser::<Shell>::new()),
                ),
        )
        .color(ColorChoice::Auto)
}
