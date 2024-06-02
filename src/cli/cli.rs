// SPDX-License-Identifier: BSD-3-Clause

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version = "v0.1.0", about = "A simple git-like VCS", long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialise a twat repository
    Init,

    /// Hash the contents of a file into a blob object
    HashFile {
        /// The file name of the file to hash
        #[arg(short, long, value_name = "FILENAME", required = true)]
        filename: String,

        /// Write the contents of the file to the object store
        #[arg(short, long)]
        write: bool,

        /// Print the hash of the compressed object file to stdout
        #[arg(short, long)]
        print: bool,
    },

    /// Print aspects of an object file to standard out
    CatFile {
        /// The BLAKE2b hash of the object to be printed
        #[arg(short, long, value_name = "HASH_DIGEST", required = true)]
        digest: String,

        /// Pretty print the contents of the object file
        #[arg(short, long)]
        pretty: bool,

        /// Print the objects type alone
        #[arg(short, long)]
        r#type: bool,

        /// Print the size of the content stored in the object file
        #[arg(short, long)]
        size: bool,
    },
}
