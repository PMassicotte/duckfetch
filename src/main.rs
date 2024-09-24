use dirs::home_dir;

use anyhow::{Context, Result};
use clap::{command, Arg, Command};
use duckup::duckup::{
    download_duckdb, duckdb_versions, extract_zip, get_latest_release, install_duckdb,
    version::check,
};

fn main() -> Result<()> {
    let mut app = command!()
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
        );

    let matches = app.clone().get_matches();

    if matches.subcommand_matches("list").is_some() {
        let available_versions = duckdb_versions()?;
        available_versions.print_versions();

        return Ok(());
    }

    if matches.subcommand_matches("check").is_some() {
        check()?;
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

        println!("Downloading DuckDB version: {} ...", requested_version);

        let (downloaded_file, temp_dir) = download_duckdb(&requested_version)?;

        println!(
            "DuckDB version {} successfully downloaded",
            requested_version
        );

        let temp_dir_str = temp_dir.path();

        extract_zip(downloaded_file, temp_dir_str)?;

        let dest_path = home_dir()
            .context("Could not find the home directory")?
            .join(".local")
            .join("bin");

        install_duckdb(temp_dir_str, &dest_path)?;

        println!(
            "DuckDB {} installed successfully in {}!",
            requested_version,
            dest_path.to_str().unwrap()
        );
        return Ok(());
    }

    // Print help message if no subcommand is provided
    if matches.subcommand_name().is_none() {
        app.print_help()?;
        println!();
    }

    Ok(())
}
