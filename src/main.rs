use anyhow::Result;
use duckup::duckup::{build_cli, check, duckdb_versions, get_latest_release, install_duckdb};

fn main() -> Result<()> {
    // Create the subcommands
    let mut app = build_cli(); // Declare `app` as mutable

    let matches = app.get_matches_mut();

    match matches.subcommand() {
        Some(("list", _)) => {
            let available_versions = duckdb_versions()?;
            available_versions.print_versions();
        }
        Some(("check", _)) => {
            check()?;
        }
        Some(("install", install_matches)) => {
            // Install a specific version. If no version provided, will try to install the latest.
            let requested_version = install_matches
                .get_one::<String>("version")
                .map_or_else(get_latest_release, |ver| Ok(ver.to_string()))?;

            install_duckdb(requested_version)?;
        }
        _ => {
            // Display help if no subcommand provided
            app.print_help()?;
            println!();
        }
    }

    Ok(())
}
