use serde::{Deserialize, Serialize};

/// Top-level application settings, serialised to and from `settings.json`.
///
/// All fields have `#[serde(default)]` so that missing keys in the file are
/// filled in with their defaults instead of causing a parse error.
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

/// Settings that control how a directory scan is performed.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScanSettings {
    #[serde(default = "ignore_pattern_default")]
    pub ignore_patterns: Vec<String>,

    #[serde(default = "bool_default")]
    pub use_gitignore: bool,

    #[serde(default = "bool_default")]
    pub scan_hidden: bool,
}
impl Default for ScanSettings {
    fn default() -> Self {
        ScanSettings {
            ignore_patterns: ignore_pattern_default(),
            use_gitignore: bool_default(),
            scan_hidden: bool_default(),
        }
    }
}

fn ignore_pattern_default() -> Vec<String> {
    vec![
        "**/.fvm",
        "**/.git",
        "**/.output",
        "**/.pnpm-store",
        "**/.pub",
        "**/build",
        "**/node_modules",
    ]
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
