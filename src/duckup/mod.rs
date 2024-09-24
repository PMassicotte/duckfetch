pub mod download;
pub mod extract;
pub mod install;
pub mod version;

pub use download::download_duckdb;
pub use extract::extract_zip;
pub use install::install_duckdb;
pub use version::{get_latest_release, list_all_versions};
