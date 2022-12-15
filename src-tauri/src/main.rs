#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{fs::File, io};

use md5::Md5;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256, Sha512};

#[derive(Debug, Serialize, Deserialize)]
struct HashData {
    md5: String,
    sha256: String,
    sha512: String,
}

impl HashData {
    fn new() -> HashData {
        HashData {
            md5: String::from(""),
            sha256: String::from(""),
            sha512: String::from(""),
        }
    }
}

#[tauri::command]
async fn get_hash(path: String, checkmd5: bool, checksha256: bool, checksha512: bool) -> HashData {
    let mut hash_data = HashData::new();
    
    if checkmd5 {
        let mut file = File::open(&path).unwrap();
        let mut hasher = Md5::new();
        io::copy(&mut file, &mut hasher).unwrap();
        let result = hasher.finalize();
        hash_data.md5 = format!("{:x}", result);
    }

    if checksha256 {
        let mut file = File::open(&path).unwrap();
        let mut hasher = Sha256::new();
        io::copy(&mut file, &mut hasher).unwrap();
        let result = hasher.finalize();
        hash_data.sha256 = format!("{:x}", result);
    }

    if checksha512 {
        let mut file = File::open(&path).unwrap();
        let mut hasher = Sha512::new();
        io::copy(&mut file, &mut hasher).unwrap();
        let result = hasher.finalize();
        hash_data.sha512 = format!("{:x}", result);
    }

    hash_data
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_hash])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
