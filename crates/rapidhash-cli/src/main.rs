//! Main CLI executable entry point.

mod args;
mod commands;
mod exit_codes;
mod i18n;

use args::{Cli, Command};
use clap::Parser;
use i18n::I18n;
use std::process;

fn main() {
    let cli = Cli::parse();
    let i18n = I18n::new(cli.locale.as_deref());

    let exit_code = match &cli.command {
        Command::Hash(hash_args) => commands::hash::run(hash_args, cli.json, &i18n),
        Command::Verify(verify_args) => commands::verify::run(verify_args, cli.json, &i18n),
        Command::Compare(compare_args) => commands::compare::run(compare_args, cli.json, &i18n),
        Command::Algorithms(algo_args) => commands::algorithms::run(algo_args, cli.json, &i18n),
    };

    process::exit(exit_code.as_i32());
}
