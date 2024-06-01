// SPDX-License-Identifier: BSD-3-Clause

mod cat;
mod cli;
mod hash;
mod init;

use clap::Parser;

use cat::cat::cat_file;
use cli::cli::{Cli, Commands};
use hash::hash::hash_file;
use init::init::init_repo;

fn main() {
    let parsed = Cli::parse();
    match &parsed.commands {
        Commands::Init => {
            println!("Initialising twat repository...");
            match init_repo() {
                Ok(()) => println!("Done!"),
                Err(e) => println!("{}", e),
            }
        }
        Commands::HashFile { filename, write } => {
            match hash_file(filename.to_string(), *write) {
                Ok(()) => {}
                Err(e) => println!("{}", e),
            }
        }
        Commands::CatBlob { blob, pretty } => {
            match cat_file(blob.to_string(), *pretty) {
                Ok(()) => {}
                Err(e) => println!("{}", e),
            }
        }
    }
    return;
}
