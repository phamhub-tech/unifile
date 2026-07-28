use std::{
    fs::{self, read_to_string},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use notify::{recommended_watcher, Watcher};
use tauri::AppHandle;

use crate::utils::get_app_data_dir;

use unifile_core::settings::{AppSettings, SettingsError};

/// Holds the current app settings and the path to the settings file on disk.
///
/// Settings are kept behind an `Arc<Mutex<_>>` so they can be shared between
/// the main thread (handling commands) and the background file-watcher thread.
pub struct AppSettingsManager {
    /// The live, shared settings value.
    pub settings: Arc<Mutex<AppSettings>>,
    /// Absolute path to `settings.json`.
    path: PathBuf,
}
impl AppSettingsManager {
    /// Loads settings from disk and starts a background file-watcher.
    ///
    /// If the settings file does not exist, an empty one is created and
    /// default settings are used. If loading fails for any other reason,
    /// defaults are used and the watcher is skipped.
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
            // Start watching only when a file exists to watch.
            manager.watch_for_changes();
        }

        manager
    }

    /// Serialises `new_settings` to JSON and writes it to the settings file.
    ///
    /// The in-memory value is **not** updated here — the background
    /// file-watcher picks up the change and updates `self.settings` instead.
    pub fn save(&self, new_settings: &AppSettings) -> Result<(), SettingsError> {
        println!("Saving new settings...");
        // serde_json::Error converts via #[from] on SettingsError::Deserialization.
        let content = serde_json::to_string_pretty(new_settings)?;
        fs::write(&self.path, content)?;
        println!("Saved settings.");
        Ok(())
    }

    /// Reads and deserialises settings from `path`.
    ///
    /// - If the file does not exist, an empty `{}` file is created and
    ///   [`AppSettings::default`] is returned.
    /// - If the file exists but cannot be parsed, a warning is printed and
    ///   [`AppSettings::default`] is returned (avoids locking the user out).
    /// - Any other IO error is returned as [`SettingsError::Io`].
    fn load_from_file<P: AsRef<Path>>(path: P) -> Result<AppSettings, SettingsError> {
        match read_to_string(&path) {
            Ok(content) => match serde_json::from_str::<AppSettings>(&content) {
                Ok(settings) => Ok(settings),
                Err(e) => {
                    eprintln!("Could not parse settings, using defaults. {e}");
                    Ok(AppSettings::default())
                }
            },
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                println!("Settings file not found. Creating empty file.");
                // io::Error converts via #[from] on SettingsError::Io.
                fs::write(&path, r#"{}"#)?;
                Ok(AppSettings::default())
            }
            Err(e) => {
                eprintln!("Could not read settings file: {e}");
                Err(SettingsError::Io(e))
            }
        }
    }

    /// Spawns a background thread that watches the settings file for external
    /// changes (e.g. the user editing it manually) and reloads it automatically.
    ///
    /// If the watcher cannot be created, a warning is printed and the thread
    /// exits cleanly — the app continues without live-reload support.
    fn watch_for_changes(&self) {
        let path = self.path.clone();
        let settings_clone = self.settings.clone();

        std::thread::spawn(move || {
            let (tx, rx) = std::sync::mpsc::channel();

            // B8 fix: don't panic if the watcher cannot be created.
            let mut watcher = match recommended_watcher(tx) {
                Ok(w) => w,
                Err(e) => {
                    eprintln!("Could not create settings file watcher: {e}");
                    return;
                }
            };

            if let Err(e) = watcher.watch(&path, notify::RecursiveMode::NonRecursive) {
                eprintln!("Could not watch settings file: {e}");
                return;
            }

            println!("Watching settings file for changes");

            for res in rx.iter() {
                match res {
                    Ok(event) => {
                        let kind = event.kind;
                        if !(kind.is_create() || kind.is_modify()) {
                            continue;
                        }

                        if let Ok(new_settings) = Self::load_from_file(&path) {
                            match settings_clone.lock() {
                                Ok(mut settings) => *settings = new_settings,
                                Err(e) => eprintln!("Settings lock poisoned: {e}"),
                            }
                        }
                    }
                    Err(e) => eprintln!("Watch error: {e}"),
                }
            }
        });
    }
}
