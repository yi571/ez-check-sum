#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::{Deserialize, Serialize};
use sha2::{Sha256, Sha512, Digest};

#[derive(Debug, Serialize, Deserialize)]
struct HashData {
    md5: String,
    sha256: String,
    sha512: String,
}

impl HashData{
    fn new() -> HashData{
        HashData{
            md5: String::from(""),
            sha256: String::from(""),
            sha512: String::from("")
        }
    }
}

#[tauri::command]
async fn get_hash(path: String) -> HashData {
    let mut hash_data = HashData::new();

    let file = std::fs::read(path).unwrap();
 
    let result = md5::compute(&file);
    hash_data.md5 = format!("{:x}", result);

    let mut hasher = Sha256::new();
    hasher.update(&file);
    let result = hasher.finalize();
    hash_data.sha256 = format!("{:x}", result);

    let mut hasher = Sha512::new();
    hasher.update(&file);
    let result = hasher.finalize();
    hash_data.sha512 = format!("{:x}", result);

    hash_data
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_hash])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
