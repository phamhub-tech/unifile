use std::io::ErrorKind;

use ignore::WalkBuilder;

use super::models::entry::FSEntry;

pub fn scan_path<F: Fn(FSEntry) -> ()>(path: String, on_update: F) {
    let mut message = "success".to_string();
    let mut has_error = false;

    let mut builder = WalkBuilder::new(&path);
    let builder = builder.standard_filters(false).hidden(false).parents(true);

    let walker = builder.build();
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
