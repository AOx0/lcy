extern crate nalgebra as na;

use std::fs::{remove_file, OpenOptions};
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

            let mut original_file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(false)
                .open(&path)
                .unwrap();

            let mut cyphered_file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&format!("{}.lcy", &path.display()))
                .unwrap();

            let mut contenidos: Vec<u8> = vec![];

            original_file.read_to_end(&mut contenidos).unwrap();

            let cifrado = lcy_lib::cypher_bytes(contenidos);

            cyphered_file.write_all(&cifrado).unwrap();

            remove_file(path).unwrap();
        }
        Commands::Decipher { path } => {
            if !path.exists() {
                eprintln!("Error: El archivo {} no existe!", path.display());
                exit(1)
            }

            let orig_path = path.as_os_str().to_str().unwrap().replace(".lcy", "");

            let mut cyphered_file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(false)
                .open(&path)
                .unwrap();

            let mut deciphered_file = OpenOptions::new()
                .read(true)
                .write(true)
                .truncate(true)
                .create(true)
                .open(&orig_path)
                .unwrap();

            let mut contenidos: Vec<u8> = vec![];

            cyphered_file.read_to_end(&mut contenidos).unwrap();

            let descifrado = lcy_lib::decipher_bytes(contenidos);

            deciphered_file.write_all(&descifrado).unwrap();
            remove_file(path).unwrap();
        }
    }
}
