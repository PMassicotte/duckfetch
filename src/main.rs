use anyhow::{Context, Result};
use clap::{command, Arg, Command};
use duckup::duckup::{
    download_duckdb, duckdb_versions, extract_zip, get_latest_release, install_duckdb,
};

fn main() -> Result<()> {
    let mut app = command!()
        .subcommand(Command::new("list").about("Lists all available DuckDB versions"))
        .subcommand(
            Command::new("install")
                .about("Installs a specific DuckDB version")
                .arg(
                    Arg::new("version")
                        .help("The version of DuckDB to install. Should be in the form of vx.y.z.")
                        .required(false),
                ),
        );

    let matches = app.clone().get_matches();

    if matches.subcommand_matches("list").is_some() {
        let available_versions = duckdb_versions()?;
        available_versions.print_versions();

        return Ok(());
    }

    // Install a specific version. If no version provided, will try to install the latest.
    if let Some(install_matches) = matches.subcommand_matches("install") {
        let requested_version = install_matches
            .get_one::<String>("version")
            .map_or_else(get_latest_release, |ver| Ok(ver.to_string()))?;

        let available_versions = duckdb_versions()?;

        // Check if the requested version exists in the available versions
        if !available_versions.contains_version(&requested_version) {
            eprintln!(
                "Error: Requested DuckDB version '{}' is not available. Choose one of the folowing:",
                requested_version
            );

            available_versions.print_versions();
            return Err(anyhow::anyhow!("Version not found"));
        }

        println!("Installing DuckDB version: {}", requested_version);

        let (downloaded_file, temp_dir) = download_duckdb(&requested_version)?;

        let temp_dir_str = temp_dir
            .path()
            .to_str()
            .context("Failed to convert temp_dir path to str")?;

        extract_zip(downloaded_file, temp_dir_str)?;
        install_duckdb(temp_dir_str, "/usr/local/bin")?;

        println!("DuckDB installed successfully!");
        return Ok(());
    }

    // Print help message if no subcommand is provided
    if matches.subcommand_name().is_none() {
        app.print_help()?;
        println!();
    }

    Ok(())
}
