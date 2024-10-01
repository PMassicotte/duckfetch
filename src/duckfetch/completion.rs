use clap::Command;
use clap_complete::Generator;
use clap_complete::{generate, Shell};
use std::io;

use crate::build_cli;

/// Generates shell completion scripts for the specified shell.
///
/// # Arguments
///
/// * `gen` - The shell completion generator.
/// * `cmd` - The command for which to generate completions.
fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

/// Handles the generation of shell completion scripts based on CLI arguments.
///
/// This function checks if the "completions" subcommand is called and if a shell is specified.
/// If so, it generates the appropriate completion script for the specified shell.
pub fn generate_completions() {
    let matches = build_cli().get_matches();

    if let Some(subcommand_matches) = matches.subcommand_matches("completions") {
        if let Some(generator) = subcommand_matches.get_one::<Shell>("shell").copied() {
            let mut cmd = build_cli();
            eprintln!("Generating completion file for {generator}...");
            print_completions(generator, &mut cmd);
        }
    }
}
