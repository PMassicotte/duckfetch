use crate::duckfetch::target::Target;
use crate::duckfetch::version::Release;
use anyhow::{Context, Result};
use std::fs::File;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use zip::read::ZipArchive;

use super::target::{Architecture, Platform};

fn target_zip_file(target: &Target, release: &Release) -> String {
    let is_nightly = !release.tag_name.starts_with("v");

    match (target.platform, target.architecture) {
        (Platform::Windows, Architecture::Amd64) => "duckdb_cli-windows-amd64.zip",
        (Platform::Windows, Architecture::Arm64) => "duckdb_cli-windows-arm64.zip",
        (Platform::MacOs, Architecture::Universal) => "duckdb_cli-osx-universal.zip",
        (Platform::Linux, Architecture::Amd64) => "duckdb_cli-linux-amd64.zip",
        (Platform::Linux, Architecture::Arm64) if is_nightly => "duckdb_cli-linux-aarch64.zip", // Specific for nightly
        (Platform::Linux, Architecture::Arm64) => "duckdb_cli-linux-arm64.zip",
        _ => panic!("Unsupported platform or architecture!"),
    }
    .to_string()
}

pub fn extract_cli(
    file_path: PathBuf,
    output_dir: &Path,
    release: &Release,
    target: &Target,
) -> Result<()> {
    let target_zip = target_zip_file(target, release);

    let downloaded_filename = file_path.file_name().unwrap().to_str().unwrap();

    let file = File::open(&file_path).context("Failed to open zip file")?;

    let mut archive = ZipArchive::new(file).context("Failed to read zip archive")?;

    // Occurring when reading a stable build zip, this contains only one binary
    if target_zip == downloaded_filename {
        println!("Extracting the main zip file...");

        archive
            .extract(output_dir)
            .context("Failed to extract the main zip file")?;
    }
    // When reading a nightly build, we need to obtain the correct embedded zip file that contains
    // the binary to use.
    else {
        let mut file_in_zip = archive.by_name(&target_zip).context(format!(
            "The file '{target_zip}' was not found inside '{downloaded_filename}'. It may not have been built yet.",
        ))?;

        let mut buffer = Vec::new();

        std::io::copy(&mut file_in_zip, &mut buffer)
            .context("Failed to copy nested zip file to buffer")?;

        let cursor = Cursor::new(buffer);

        let mut nested_archive =
            ZipArchive::new(cursor).context("Failed to read nested zip archive")?;

        nested_archive
            .extract(output_dir)
            .context("Failed to extract the nested zip file")?;
    }

    Ok(())
}
