use std::fs;
use std::path::PathBuf;

use tauri::{AppHandle, Manager};

pub fn get_app_data_dir(handle: &AppHandle) -> PathBuf {
    let path = handle
        .path()
        .app_data_dir()
        .expect("Could not get data directory");

    if !path.exists() {
        println!("Path doesn't exist. Creating path {:?}...", path);
        fs::create_dir(&path)
            .map_err(|err| format!("Could not create data directory: {err}"))
            .unwrap();
        println!("Path created");
    }

    path
}
