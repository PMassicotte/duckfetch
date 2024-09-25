use anyhow::{Context, Result};
use duckup::duckup::{build_cli, check, duckdb_versions, install_duckdb};

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
        Some(("install", _)) => {
            let available_versions = duckdb_versions()?;
            let tag_names = available_versions.releases();
            let selected_tag = inquire::Select::new(
                "Select the DuckDB version to install (Esc to cancel): ",
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
