pub mod cli;
pub mod download;
pub mod extract;
pub mod install;
pub mod version;

pub use cli::build_cli;
pub use download::download_duckdb;
pub use extract::extract_zip;
pub use install::install_duckdb;
pub use version::{check, duckdb_versions, get_latest_release};
