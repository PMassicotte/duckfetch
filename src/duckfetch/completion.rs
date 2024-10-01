use clap::Command;
use clap_complete::Generator;
use clap_complete::{generate, Shell};
use std::io;

use crate::build_cli;

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

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
