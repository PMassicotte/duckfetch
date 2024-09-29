use crate::duckfetch::download_duckdb;
use crate::duckfetch::duckdb_versions;
use crate::duckfetch::extract_zip;
use crate::duckfetch::version::Release;
use anyhow::Ok;
use anyhow::{Context, Result};
use dirs::home_dir;
use inquire::Confirm;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

/// Returns the appropriate destination directory for the DuckDB binary based on the operating system.
///
/// On Windows, it uses `AppData\Local\bin` under the user's home directory.
/// On Linux/macOS, it uses `.local/bin` under the user's home directory.
///
/// # Errors
///
/// Returns an error if the home directory cannot be determined.
fn get_dest_dir() -> Result<PathBuf> {
    let home_dir = home_dir().context("Could not find the home directory")?;
    let dest_dir = if cfg!(target_os = "windows") {
        home_dir
            .join("AppData")
            .join("Local")
            .join("Programs")
            .join("Duckdb")
    } else {
        home_dir.join(".local").join("bin")
    };

    Ok(dest_dir)
}

/// Installs the DuckDB binary by moving it from the output directory to the install directory.
///
/// # Arguments
///
/// * `temp_unzip_dir` - A `Path` representing the path to the directory containing the DuckDB binary.
/// * `dest_path` - A `Path` representing the path to the directory where the DuckDB binary should be installed.
///
/// # Returns
///
/// * `Result<()>` - An empty result if successful, or an error if the installation fails.
fn install(temp_unzip_dir: &Path, dest_path: &Path) -> Result<()> {
    let entries = fs::read_dir(temp_unzip_dir).context("Failed to read directory")?;
    let mut found = false;

    for entry in entries {
        let entry = entry.context("Failed to read directory entry")?;
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();

        if file_name_str == "duckdb" || file_name_str == "duckdb.exe" {
            let src = entry.path();
            let dest = Path::new(dest_path).join(file_name_str.to_string());
            fs::rename(&src, &dest).context("Failed to move DuckDB binary")?;
            found = true;
            break;
        }
    }

    if !found {
        return Err(anyhow::anyhow!(
            "Neither duckdb nor duckdb.exe found in the source directory"
        ));
    }

    Ok(())
}

/// Installs the specified version of DuckDB.
///
/// This function performs the following steps:
/// 1. Retrieves the list of available DuckDB versions.
/// 2. Checks if the requested version exists in the available versions.
/// 3. Downloads the requested version of DuckDB.
/// 4. Extracts the downloaded file.
/// 5. Installs DuckDB to the user's local bin directory.
///
/// # Arguments
///
/// * `requested_version` - A string specifying the version of DuckDB to install.
///
/// # Errors
///
/// This function will return an error if:
/// - The list of available DuckDB versions cannot be retrieved.
/// - The requested version is not found in the available versions.
/// - The DuckDB download fails.
/// - The downloaded file cannot be extracted.
/// - The home directory cannot be found.
/// - The installation process fails.
///
/// # Returns
///
/// This function returns `Ok(())` if the installation completes successfully.
pub fn install_duckdb(requested_release: &Release) -> Result<()> {
    let available_versions = duckdb_versions()?;

    // Check if the requested version exists in the available versions
    if !available_versions.contains_version(&requested_release.tag_name) {
        eprintln!(
            "Error: Requested DuckDB version '{}' is not available. Choose one of the following:",
            requested_release.tag_name
        );

        available_versions.print_versions();

        return Err(anyhow::anyhow!("Version not found"));
    }

    println!(
        "Downloading DuckDB version: {} ...",
        requested_release.tag_name
    );

    let (downloaded_file, temp_dir) = download_duckdb(requested_release)?;

    println!(
        "DuckDB version {} successfully downloaded",
        &requested_release.tag_name
    );

    let temp_dir_str = temp_dir.path();

    extract_zip(downloaded_file, temp_dir_str)?;

    // Determine the destination path based on the platform
    let dest_dir = get_dest_dir()?;

    // Ask the user if the destination folder should be created
    if !dest_dir.exists() {
        let answer = Confirm::new(&format!(
            "{} does not exist. Would you like to create it?",
            dest_dir.display()
        ))
        .with_default(false)
        .with_help_message("Select 'yes' to create the folder")
        .prompt()?;

        if answer {
            // Create the directory if the user agreed
            fs::create_dir_all(&dest_dir)
                .context(format!("Failed to create directory {}", dest_dir.display()))?;
            println!("Directory {} created successfully.", dest_dir.display());
        } else {
            return Err(anyhow::anyhow!(
                "Aborting installation as the destination directory was not created."
            ));
        }
    }

    install(temp_dir_str, &dest_dir)?;

    println!(
        "DuckDB {} installed successfully in {}!",
        requested_release.tag_name,
        dest_dir.to_str().unwrap()
    );

    Ok(())
}
