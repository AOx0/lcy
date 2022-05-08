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
            let bytes: DMatrix<f32> = dmatrix![
                 0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0, 31.0 ;
                 32.0, 33.0, 34.0, 35.0, 36.0, 37.0, 38.0, 39.0, 40.0, 41.0, 42.0, 43.0, 44.0, 45.0, 46.0, 47.0, 48.0, 49.0, 50.0, 51.0, 52.0, 53.0, 54.0, 55.0, 56.0, 57.0, 58.0, 59.0, 60.0, 61.0, 62.0, 63.0;
                 64.0, 65.0, 66.0, 67.0, 68.0, 69.0, 70.0, 71.0, 72.0, 73.0, 74.0, 75.0, 76.0, 77.0, 78.0, 79.0, 80.0, 81.0, 82.0, 83.0, 84.0, 85.0, 86.0, 87.0, 88.0, 89.0, 90.0, 91.0, 92.0, 93.0, 94.0, 95.0;
                 96.0, 97.0, 98.0, 99.0, 100.0, 101.0, 102.0, 103.0, 104.0, 105.0, 106.0, 107.0, 108.0, 109.0, 110.0, 111.0, 112.0, 113.0, 114.0, 115.0, 116.0, 117.0, 118.0, 119.0, 120.0, 121.0, 122.0, 123.0, 124.0, 125.0, 126.0, 127.0 ;
                 128.0, 129.0, 130.0, 131.0, 132.0, 133.0, 134.0, 135.0, 136.0, 137.0, 138.0, 139.0, 140.0, 141.0, 142.0, 143.0, 144.0, 145.0, 146.0, 147.0, 148.0, 149.0, 150.0, 151.0, 152.0, 153.0, 154.0, 155.0, 156.0, 157.0, 158.0, 159.0 ;
                 160.0, 161.0, 162.0, 163.0, 164.0, 165.0, 166.0, 167.0, 168.0, 169.0, 170.0, 171.0, 172.0, 173.0, 174.0, 175.0, 176.0, 177.0, 178.0, 179.0, 180.0, 181.0, 182.0, 183.0, 184.0, 185.0, 186.0, 187.0, 188.0, 189.0, 190.0, 191.0 ;
                 192.0, 193.0, 194.0, 195.0, 196.0, 197.0, 198.0, 199.0, 200.0, 201.0, 202.0, 203.0, 204.0, 205.0, 206.0, 207.0, 208.0, 209.0, 210.0, 211.0, 212.0, 213.0, 214.0, 215.0, 216.0, 217.0, 218.0, 219.0, 220.0, 221.0, 222.0, 223.0 ;
                 224.0, 225.0, 226.0, 227.0, 228.0, 229.0, 230.0, 231.0, 232.0, 233.0, 234.0, 235.0, 236.0, 237.0, 238.0, 239.0, 240.0, 241.0, 242.0, 243.0, 244.0, 245.0, 246.0, 247.0, 248.0, 249.0, 250.0, 251.0, 252.0, 253.0, 254.0, 255.0
            ];

            if !path.exists() {
                eprintln!("Error: El archivo {} no existe!", path.display());
                exit(1)
            }

            let mut file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(false)
                .open(&path)
                .unwrap();

            let mut file2 = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&format!("{}.lcy", &path.display()))
                .unwrap();

            let mut contenidos: Vec<u8> = vec![];

            file.read_to_end(&mut contenidos).unwrap();

            let mat = met1_armar_matriz(&mut rng);
            let mut key = &bytes * &mat;
            let inv_key = mat.clone().try_inverse().unwrap();

            for i in 0..8usize {
                for j in 0..32usize {
                    key[(i, j)] = key[(i, j)].rem_euclid(256.0);
                    map.insert(bytes[(i, j)] as u8, key[(i, j)] as u8);
                }
            }

            for i_byte in 0..contenidos.len() {
                contenidos[i_byte] = *map.get(&contenidos[i_byte]).unwrap();
            }

            let mut inv_key_to_write = vec![];
            let mut bytes_final_to_write = vec![];
            for i in 0..32 {
                for j in 0..32 {
                    let indicador = if inv_key[(i, j)] < 0.0 { 1u8 } else { 0u8 };
                    let val = if inv_key[(i, j)] < 0.0 {
                        -1.0 * inv_key[(i, j)]
                    } else {
                        inv_key[(i, j)]
                    };

                    inv_key_to_write.push(indicador);
                    inv_key_to_write.push(val as u8);
                }
            }

            for i in 0..8 {
                for j in 0..32 {
                    bytes_final_to_write.push(key[(i, j)] as u8);
                }
            }

            file2.write_all(&[66u8, 60u8, 10u8, 255u8]).unwrap();
            file2.write_all(&inv_key_to_write).unwrap();
            file2.write_all(&bytes_final_to_write).unwrap();
            file2.write_all(&contenidos).unwrap();

            remove_file(path).unwrap();
        }
        Commands::Decipher { path } => {
            if !path.exists() {
                eprintln!("Error: El archivo {} no existe!", path.display());
                exit(1)
            }

            let orig_path = path.as_os_str().to_str().unwrap().replace(".lcy", "");

            let mut file_ci = OpenOptions::new()
                .read(true)
                .write(true)
                .create(false)
                .open(&path)
                .unwrap();

            let mut file = OpenOptions::new()
                .read(true)
                .write(true)
                .truncate(true)
                .create(true)
                .open(&orig_path)
                .unwrap();

            let mut contenidos: Vec<u8> = vec![];

            file_ci.read_to_end(&mut contenidos).unwrap();

            if contenidos[0..0x4] != [66u8, 60u8, 10u8, 255u8] {
                eprintln!("Error: No es un archivo válido cifrado");
                exit(1);
            }

            let mut inv = dmatrix![].resize(32, 32, 0.0);
            let mut key = dmatrix![].resize(8, 32, 0.0);

            let contenidos = &contenidos[0x4..];

            let mut k = 0;
            for i in 0..32 {
                for j in 0..32 {
                    let neg = contenidos[k] == 0x1;
                    let val = (contenidos[k + 1] as i32) * if neg { -1 } else { 1 };

                    inv[(i, j)] = val as f32;
                    k += 2;
                }
            }

            let contenidos = &contenidos[k..];

            let mut k = 0;
            for i in 0..8 {
                for j in 0..32 {
                    key[(i, j)] = contenidos[k] as f32;
                    k += 1;
                }
            }

            let mut orig = &key * &inv;

            for i in 0..8usize {
                for j in 0..32usize {
                    orig[(i, j)] = orig[(i, j)].rem_euclid(256.0);
                    map.insert(key[(i, j)] as u8, orig[(i, j)] as u8);
                }
            }

            let mut contenidos = contenidos[k..].to_vec();

            for i_byte in 0..contenidos.len() {
                contenidos[i_byte] = *map.get(&contenidos[i_byte]).unwrap();
            }

            file.write_all(&contenidos).unwrap();
            remove_file(path).unwrap();
        }
    }
}
