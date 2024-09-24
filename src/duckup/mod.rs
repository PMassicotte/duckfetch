/// Main module for the DuckDB utility, providing functionalities for downloading,
/// extracting, installing, and managing DuckDB versions.
pub mod download;
pub mod extract;
pub mod install;
pub mod version;

pub use download::download_duckdb;
pub use extract::extract_zip;
pub use install::install_duckdb;
pub use version::{duckdb_versions, get_latest_release};

