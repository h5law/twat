// SPDX-License-Identifier: BSD-3-Clause

mod cli;
mod init;
mod objects;
mod utils;

use std::env;

use clap::Parser;

use cli::cli::{Cli, Commands};
use init::init::init_repo;
use objects::cat::cat_file;
use objects::hash::hash_file;

fn main() {
    let parsed = Cli::parse();
    match &parsed.commands {
        Commands::Init => match init_repo() {
            Ok(()) => {
                let path = env::current_dir()
                    .expect("[twat]: unable to get current directory");
                let dir_path =
                    path.clone().into_os_string().into_string().expect(
                        "[twat]: error converting directory OsString to String",
                    );
                println!(
                    "Initialised empty twat repository in {:?}",
                    dir_path + "/.twat"
                );
            }
            Err(e) => {
                println!("{}", e);
            }
        },
        Commands::HashFile {
            filename,
            write,
            print,
        } => match hash_file(filename.to_string(), *write, *print) {
            Ok(()) => {}
            Err(e) => println!("{}", e),
        },
        Commands::CatFile {
            digest,
            pretty,
            r#type,
            size,
        } => match cat_file(digest.to_string(), *pretty, *r#type, *size) {
            Ok(()) => {}
            Err(e) => println!("{}", e),
        },
    }
}
