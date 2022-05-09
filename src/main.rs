extern crate nalgebra as na;

use std::collections::HashMap;
use std::fs::{remove_file, OpenOptions};
use std::io::{Read, Write};
use std::process::exit;

use na::{dmatrix, DMatrix};

pub use clap::{Parser, Subcommand};
use rand::rngs::ThreadRng;
use rand::Rng;
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

pub fn met1_armar_matriz(rng: &mut ThreadRng) -> DMatrix<f32> {
    let mut resultado: DMatrix<f32> = dmatrix![].resize(32, 32, 0.0);

    let mut switch: [bool; 32] = [false; 32];
    let mut neg: [bool; 32] = [false; 32];

    for col in 1..32 / 2 {
        if rng.gen::<u8>() % 2 == 0 {
            switch[col] = true;
            switch[31 - (col - 1)] = true;
        }

        if rng.gen::<u8>() % 5 == 0 {
            neg[col] = true;
            neg[31 - (col - 1)] = true;
        }
    }

    resultado[(0, 0)] = 1.0;

    for col in 1..32 {
        if switch[col] {
            resultado[(col, 31 - (col - 1))] = if neg[col] { 1.0 } else { -1.0 };
        } else {
            resultado[(col, col)] = if neg[col] { 1.0 } else { -1.0 };
        }
    }

    resultado
}

fn main() {
    let mut rng = rand::thread_rng();
    let args: Args = Args::parse();

    let mut map: HashMap<u8, u8> = HashMap::new();

    match args.command {
        Commands::Cypher { path } => {
            let bytes = craft_bytes_matrix();

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

            let transformation = met1_armar_matriz(&mut rng);
            let mut cyphered_bytes = &bytes * &transformation;
            let transformation_inverse = transformation.clone().try_inverse().unwrap();

            for i in 0..8usize {
                for j in 0..32usize {
                    cyphered_bytes[(i, j)] = cyphered_bytes[(i, j)].rem_euclid(256.0);
                    map.insert(bytes[(i, j)] as u8, cyphered_bytes[(i, j)] as u8);
                }
            }

            for i_byte in 0..contenidos.len() {
                contenidos[i_byte] = *map.get(&contenidos[i_byte]).unwrap();
            }

            let mut inv_key_to_write = vec![];
            let mut bytes_final_to_write = vec![];
            for i in 0..32 {
                for j in 0..32 {
                    let indicador = if transformation_inverse[(i, j)] < 0.0 { 1u8 } else { 0u8 };
                    let val = if transformation_inverse[(i, j)] < 0.0 {
                        -1.0 * transformation_inverse[(i, j)]
                    } else {
                        transformation_inverse[(i, j)]
                    };

                    inv_key_to_write.push(indicador);
                    inv_key_to_write.push(val as u8);
                }
            }

            for i in 0..8 {
                for j in 0..32 {
                    bytes_final_to_write.push(cyphered_bytes[(i, j)] as u8);
                }
            }

            cyphered_file.write_all(&[66u8, 60u8, 10u8, 255u8]).unwrap();
            cyphered_file.write_all(&inv_key_to_write).unwrap();
            cyphered_file.write_all(&bytes_final_to_write).unwrap();
            cyphered_file.write_all(&contenidos).unwrap();

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

            if contenidos[0..0x4] != [66u8, 60u8, 10u8, 255u8] {
                eprintln!("Error: No es un archivo válido cifrado");
                exit(1);
            }

            let mut transformation_inverse = dmatrix![].resize(32, 32, 0.0);
            let mut cyphered_bytes = dmatrix![].resize(8, 32, 0.0);

            let contenidos = &contenidos[0x4..];

            let mut k = 0;
            for i in 0..32 {
                for j in 0..32 {
                    let neg = contenidos[k] == 0x1;
                    let val = (contenidos[k + 1] as i32) * if neg { -1 } else { 1 };

                    transformation_inverse[(i, j)] = val as f32;
                    k += 2;
                }
            }

            let contenidos = &contenidos[k..];

            let mut k = 0;
            for i in 0..8 {
                for j in 0..32 {
                    cyphered_bytes[(i, j)] = contenidos[k] as f32;
                    k += 1;
                }
            }

            let mut orig = &cyphered_bytes * &transformation_inverse;

            for i in 0..8usize {
                for j in 0..32usize {
                    orig[(i, j)] = orig[(i, j)].rem_euclid(256.0);
                    map.insert(cyphered_bytes[(i, j)] as u8, orig[(i, j)] as u8);
                }
            }

            let mut contenidos = contenidos[k..].to_vec();

            for i_byte in 0..contenidos.len() {
                contenidos[i_byte] = *map.get(&contenidos[i_byte]).unwrap();
            }

            deciphered_file.write_all(&contenidos).unwrap();
            remove_file(path).unwrap();
        }
    }
}

pub fn craft_bytes_matrix() -> DMatrix<f32> {
    let mut bytes: DMatrix<f32> = dmatrix![].resize(8, 32, 0.0);

    let mut k = 0;
    for i in 0..8 {
        for j in 0..32 {
            bytes[(i, j)] = k as f32;
            k += 1;
        }
    }
    bytes
}
