extern crate nalgebra as na;

use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::process::exit;

pub use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[clap(version)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Cypher any file with lcy
    #[clap(visible_alias = "c")]
    Cypher {
        /// Name of the file to cypher
        path: PathBuf,
    },
    /// Decipher a lcy file
    #[clap(visible_alias = "d")]
    Decipher {
        /// Name of the file to decipher
        path: PathBuf,
    },
}

fn main() {
    match Args::parse().command {
        Commands::Cypher { path } => {
            if !path.exists() {
                eprintln!("Error: El archivo {} no existe!", path.display());
                exit(1)
            }

            let mut contenidos: Vec<u8> = vec![];
            {
                let mut original_file = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(false)
                    .open(&path)
                    .unwrap();

                original_file.read_to_end(&mut contenidos).unwrap();
            }

            let mut cyphered_file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&path)
                .unwrap();

            let cifrado = lcy_lib::cypher_bytes(contenidos);

            cyphered_file.write_all(&cifrado).unwrap();
        }
        Commands::Decipher { path } => {
            if !path.exists() {
                eprintln!("Error: El archivo {} no existe!", path.display());
                exit(1)
            }

            let contenidos: Vec<u8> = std::fs::read(&path).unwrap();

            let descifrado = lcy_lib::decipher_bytes(contenidos);

            let mut deciphered_file = OpenOptions::new()
                .read(true)
                .write(true)
                .truncate(true)
                .create(true)
                .open(&path)
                .unwrap();

            deciphered_file.write_all(&descifrado).unwrap();
        }
    }
}
