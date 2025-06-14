use std::io::ErrorKind;

use ignore::{overrides::OverrideBuilder, WalkBuilder};

use crate::settings::models::ScanSettings;

use super::models::entry::FSEntry;

pub fn scan_path<F: Fn(FSEntry) -> ()>(path: String, scan_settings: ScanSettings, on_update: F) {
    let mut message = "success".to_string();
    let mut has_error = false;

    let mut builder = WalkBuilder::new(&path);
    let mut builder = builder.standard_filters(false).parents(true);

    if scan_settings.use_gitignore {
        builder = builder.git_ignore(true).git_exclude(true).git_global(true);
    }
    if scan_settings.scan_hidden {
        builder = builder.hidden(false);
    }

    let mut override_builder = OverrideBuilder::new(&path);
    for pattern in &scan_settings.ignore_patterns {
        match override_builder.add(&format!("!{pattern}")) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("An error occured building glob: {pattern}");
                eprintln!("{:?}", e);
            }
        }
    }

    match override_builder.build() {
        Ok(overrides) => {
            builder.overrides(overrides);
        },
        Err(e) => {
            eprintln!("Could not create overrided: {:?}", e);
        },
    };

    let walker = builder.build();
    println!("Scanning {path}");
    for result in walker {
        match result {
            Ok(entry) => {
                let entry_path = entry.path().to_string_lossy().to_string();
                if entry_path == path {
                    println!("Ignoring root path: {path}");
                    continue;
                }

                let fs_entry = match FSEntry::from_entry(&entry) {
                    Ok(entry) => entry,
                    Err(e) => match e.kind() {
                        ErrorKind::PermissionDenied => {
                            eprintln!("Permission deined: {:?}", entry);
                            continue;
                        }
                        ErrorKind::NotFound => {
                            eprintln!("Path not found: {:?}", entry);
                            continue;
                        }
                        _ => {
                            eprintln!("FS Entry Error: {:?}", e);
                            message = "Uknown error".to_string();
                            has_error = true;
                            break;
                        }
                    },
                };

                on_update(fs_entry);
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
            }
        }
    }
}
