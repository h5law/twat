// SPDX-License-Identifier: BSD-3-Clause

mod cli;
mod init;

use clap::Parser;

use cli::cli::{Cli, Commands};
use init::init::init_repo;

fn main() {
    let parsed = Cli::parse();
    match &parsed.commands {
        Commands::Init => {
            println!("Initialising twat repository...");
            match init_repo() {
                Ok(()) => println!("Success!"),
                Err(e) => panic!("{}", e),
            }
        }
        Commands::HashFile { filename, write } => {
            println!("hash-file: {:?} {write}", filename);
        }
        Commands::CatBlob { blob, pretty } => {
            println!("cat-blob: {pretty} {:?}", blob);
        }
    }
}
