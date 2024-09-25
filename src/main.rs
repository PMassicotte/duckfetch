use anyhow::{Context, Result};
use duckup::duckup::{build_cli, check, duckdb_versions, get_latest_release, install_duckdb};

fn main() -> Result<()> {
    let mut app = build_cli();
    let matches = app.get_matches_mut();

    match matches.subcommand() {
        Some(("list", _)) => {
            duckdb_versions()?.print_versions();
        }
        Some(("check", _)) => {
            check()?;
        }
        Some(("install", install_matches)) => {
            let requested_version = install_matches
                .get_one::<String>("version")
                .map_or_else(get_latest_release, |ver| Ok(ver.to_string()))?;

            install_duckdb(requested_version)?;
        }
        Some(("install-from", _)) => {
            let available_versions = duckdb_versions()?;
            let tag_names = available_versions.releases();
            let selected_tag = inquire::Select::new(
                "Select DucDB version to install (Esc to cancel): ",
                tag_names,
            )
            .prompt()
            .context("Error")?;

            install_duckdb(selected_tag)?;
        }
        _ => {
            app.print_help()?;
            println!();
        }
    }

    Ok(())
}
