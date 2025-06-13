use std::{
    fs::{self, read_to_string},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use notify::{recommended_watcher, Watcher};
use serde::{Deserialize, Serialize};
use tauri::AppHandle;

use crate::utils::get_app_data_dir;

pub struct AppSettingsManager {
    pub settings: Arc<Mutex<AppSettings>>,
    path: PathBuf,
}
impl AppSettingsManager {
    /// Loads the app settings to use
    pub fn new(handle: &AppHandle) -> Self {
        let json_path = get_app_data_dir(handle).join("settings.json");
        let mut should_watch_settings = true;
        let settings = match Self::load_from_file(&json_path) {
            Ok(s) => s,
            Err(e) => {
                // At this point the settings files does not exist. No need to watch it
                eprintln!("Failed to load settings: {:?} {e}", json_path);
                should_watch_settings = false;
                AppSettings::default()
            }
        };

        let manager = Self {
            settings: Arc::new(Mutex::new(settings.clone())),
            path: json_path,
        };

        if should_watch_settings {
            // Only watch for changes if we successfully created the settings file
            manager.watch_for_changes();
        }

        manager
    }

    /// Saves the app settings
    ///
    /// The current settings will be updated by the watcher, not this function
    pub fn save(&self, new_settings: &AppSettings) -> Result<(), Box<dyn std::error::Error>> {
        println!("Saving new settings...");

        // TODO: Handle errors properly
        let content = serde_json::to_string_pretty(new_settings)?;
        fs::write(&self.path, content)?;
        println!("Saved settings.");
        Ok(())
    }

    /// Loads the settings from the provided file path
    ///
    /// Returns an io::Error if the file could not be read or if a new empty file
    /// could not be created
    fn load_from_file<P: AsRef<Path>>(path: P) -> Result<AppSettings, std::io::Error> {
        match read_to_string(&path) {
            Ok(content) => match serde_json::from_str::<AppSettings>(&content) {
                Ok(settings) => Ok(settings),
                Err(e) => {
                    eprintln!("Could not deserialize settings, using default. {e}");
                    eprintln!("Loaded settings from file: {content}");
                    Ok(AppSettings::default())
                }
            },
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => {
                    println!("Settings file not found. Creating empty file");

                    // TODO: Find a better way to handle the write failed error
                    match fs::write(&path, r#"{}"#) {
                        Ok(_) => Ok(AppSettings::default()),
                        Err(e) => {
                            eprintln!("Failed to write settings file: {e}");
                            return Err(e);
                        }
                    }
                }
                kind => {
                    eprintln!("Could not read settings file: {kind}");
                    Err(e)
                }
            },
        }
    }

    fn watch_for_changes(&self) {
        let path = self.path.clone();
        let settings_clone = self.settings.clone();

        std::thread::spawn(move || {
            let (tx, rx) = std::sync::mpsc::channel();
            let mut watcher = recommended_watcher(tx).unwrap();
            watcher
                .watch(&path, notify::RecursiveMode::NonRecursive)
                .expect("Watch settings file");
            println!("Watching settings file for changes");

            for res in rx.iter() {
                match res {
                    Ok(event) => {
                        let kind = event.kind;
                        if !(kind.is_create() || kind.is_modify()) {
                            continue;
                        }

                        if let Ok(new_settings) = Self::load_from_file(&path) {
                            let mut settings = settings_clone.lock().unwrap();
                            *settings = new_settings;
                        }
                    }
                    Err(e) => eprintln!("Watch error: {:?}", e),
                }
            }
        });
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppSettings {
    #[serde(default = "theme_default")]
    pub theme: String,

    #[serde(default = "language_default")]
    pub language: String,

    #[serde(default)]
    pub scan: ScanSettings,
}

impl Default for AppSettings {
    fn default() -> Self {
        AppSettings {
            theme: theme_default(),
            language: language_default(),
            scan: ScanSettings::default(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScanSettings {
    #[serde(default = "ignore_pattern_default")]
    pub ignore_patterns: Vec<String>,

    #[serde(default = "bool_default")]
    pub use_gitignore: bool,
}
impl Default for ScanSettings {
    fn default() -> Self {
        ScanSettings {
            ignore_patterns: ignore_pattern_default(),
            use_gitignore: bool_default(),
        }
    }
}

fn ignore_pattern_default() -> Vec<String> {
    vec!["**/.git", "**/.output", "**/node_modules", "*.lock"]
        .iter()
        .map(|pattern| pattern.to_string())
        .collect()
}

fn bool_default() -> bool {
    true
}

fn theme_default() -> String {
    "light".to_string()
}

fn language_default() -> String {
    "en".to_string()
}
