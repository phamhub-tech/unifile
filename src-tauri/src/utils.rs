use std::fs;
use std::path::PathBuf;

use tauri::{AppHandle, Manager};

/// Returns the platform-specific application data directory.
///
/// If the directory does not exist yet, it is created (including any missing
/// parent directories). Panics if the path cannot be determined or created,
/// since the app cannot run without a data directory.
pub fn get_app_data_dir(handle: &AppHandle) -> PathBuf {
    let path = handle
        .path()
        .app_data_dir()
        .expect("Could not get data directory");

    if !path.exists() {
        println!("Path doesn't exist. Creating path {:?}...", path);
        fs::create_dir_all(&path)
            .map_err(|err| format!("Could not create data directory: {err}"))
            .unwrap();
        println!("Path created");
    }

    path
}
