pub mod cli;
pub mod completion;
pub mod download;
pub mod extract;
pub mod install;
pub mod target;
pub mod url;
pub mod version;

pub use cli::build_cli;
pub use download::download_duckdb;
pub use extract::extract_cli;
pub use install::install_duckdb;
pub use version::{check, duckdb_versions};
