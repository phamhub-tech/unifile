use std::io::ErrorKind;

use ignore::WalkBuilder;
use tauri::ipc::Channel;

use crate::api::{ApiError, ApiResponse};
use crate::fs::models::scan::ScanEvent;
use crate::fs::models::{drive::Drive, entry::FSEntry};
use crate::{api_error, api_response};

use super::controllers;

#[tauri::command]
pub fn get_drives() -> Result<ApiResponse<Vec<Drive>>, ApiError> {
    let drives = sysinfo::Disks::new_with_refreshed_list()
        .iter()
        .map(|disk| Drive::from_disk(disk))
        .collect();

    Ok(api_response!(drives))
}

#[tauri::command]
pub fn get_entries(path: String) -> Result<ApiResponse<Vec<FSEntry>>, ApiError> {
    let mut message = "Success".to_string();
    let mut has_error = false;
    let mut entries = Vec::new();

    let mut builder = WalkBuilder::new(&path);
    let builder = builder.standard_filters(false).hidden(false).parents(true);

    let walker = builder.max_depth(Some(1)).build();
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
                entries.push(fs_entry);
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
            }
        }
    }

    if has_error {
        return Err(api_error!(message));
    }

    Ok(api_response!(entries, message, has_error))
}

#[tauri::command]
pub async fn scan_path(
    path: String,
    on_event: Channel<ScanEvent>,
) -> Result<ApiResponse<Option<()>>, ApiError> {
    on_event.send(ScanEvent::Started {}).unwrap();

    controllers::scan_path(path, |entry| {
        on_event.send(ScanEvent::Progress { entry }).unwrap();
    });

    on_event.send(ScanEvent::Finished {}).unwrap();
    Ok(api_response!(None))
}
