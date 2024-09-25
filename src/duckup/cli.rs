use clap::{command, Arg, Command};

/// Builds the command-line interface (CLI) for the DuckDB utility.
///
/// This function sets up the CLI with the following subcommands:
/// - `list`: Lists all available DuckDB versions.
/// - `check`: Compares the installed version of DuckDB with the latest release.
/// - `install`: Installs a specific DuckDB version or the latest version if none is provided.
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
                .about("Installs a specific DuckDB version or the latest version if none provided")
                .arg(
                    Arg::new("version")
                        .help("The version of DuckDB to install. Should be in the form of vx.y.z.")
                        .required(false),
                ),
        )
        .subcommand(
            Command::new("install-from").about("Install DuckDB using a list of available versions"),
        )
}
