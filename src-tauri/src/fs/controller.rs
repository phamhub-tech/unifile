use ignore::{overrides::OverrideBuilder, WalkBuilder};
use tauri::ipc::Channel;

use crate::fs::error::FsError;
use crate::fs::models::drive::Drive;
use crate::fs::models::scan::ScanEvent;
use crate::settings::models::ScanSettings;

use super::models::entry::FSEntry;

/// Returns all mounted drives detected on the system.
pub fn get_drives() -> Vec<Drive> {
    sysinfo::Disks::new_with_refreshed_list()
        .iter()
        .map(|disk| Drive::from_disk(disk))
        .collect()
}

/// Returns the immediate children of `path` (depth = 1).
///
/// Permission-denied and not-found entries are silently skipped.
/// Any other IO error stops enumeration and is returned as an `Err`.
pub fn get_entries(path: &str) -> Result<Vec<FSEntry>, FsError> {
    let mut entries = Vec::new();
    let walker = WalkBuilder::new(path)
        .standard_filters(false)
        .hidden(false)
        .parents(true)
        .max_depth(Some(1))
        .build();

    for result in walker {
        let entry = match result {
            Ok(e) => e,
            Err(e) => {
                eprintln!("Walk error: {e}");
                continue;
            }
        };

        // The walker returns the root path as its first result — skip it.
        if entry.path() == std::path::Path::new(path) {
            continue;
        }

        match FSEntry::from_entry(&entry) {
            Ok(e) => entries.push(e),
            Err(FsError::PermissionDenied { path }) => {
                eprintln!("Permission denied: {path}");
                continue;
            }
            Err(FsError::NotFound { path }) => {
                eprintln!("Not found: {path}");
                continue;
            }
            Err(e) => return Err(e),
        }
    }

    Ok(entries)
}

/// Walks `path` recursively and calls `on_update` for every entry found.
///
/// `on_update` should return `Ok(())` to continue, or `Err(FsError::ChannelClosed)`
/// to abort the walk early (e.g. when the IPC channel to the frontend is gone).
///
/// # Errors
///
/// Returns `Err` if an unexpected IO error occurs (not permission-denied or
/// not-found, which are silently skipped). Also returns `Err` if `on_update`
/// returns `Err`.
///
/// # Separation of concerns
///
/// This function never touches [`ApiResponse`]. All error signalling goes
/// through the `Result` return value and the `on_update` callback's `Result`.
///
/// [`ApiResponse`]: crate::api::ApiResponse
pub fn scan_path<F>(path: String, scan_settings: ScanSettings, on_update: F) -> Result<(), FsError>
where
    F: Fn(FSEntry) -> Result<(), FsError>,
{
    let mut builder = WalkBuilder::new(&path);
    builder.standard_filters(false).parents(true);

    if scan_settings.use_gitignore {
        builder.git_ignore(true).git_exclude(true).git_global(true);
    }
    if scan_settings.scan_hidden {
        builder.hidden(false);
    }

    let mut override_builder = OverrideBuilder::new(&path);
    for pattern in &scan_settings.ignore_patterns {
        // ignore::Error converts via #[from] on FsError::OverrideBuild — no map_err needed.
        override_builder.add(&format!("!{pattern}"))?;
    }
    // ignore::Error converts via #[from] here too.
    let overrides = override_builder.build()?;
    builder.overrides(overrides);

    println!("Scanning {path}");
    for result in builder.build() {
        let entry = match result {
            Ok(e) => e,
            Err(e) => {
                eprintln!("Walk error: {e}");
                continue;
            }
        };

        // The root path itself is returned by the walker as the first entry — skip it.
        if entry.path() == std::path::Path::new(&path) {
            continue;
        }

        let fs_entry = match FSEntry::from_entry(&entry) {
            Ok(e) => e,
            // Non-fatal: log and continue the walk.
            Err(FsError::PermissionDenied { path }) => {
                eprintln!("Permission denied: {path}");
                continue;
            }
            Err(FsError::NotFound { path }) => {
                eprintln!("Not found (file removed during scan): {path}");
                continue;
            }
            // Fatal: stop the scan and report to the caller.
            Err(e) => return Err(e),
        };

        // Propagates FsError::ChannelClosed if the frontend disconnected.
        on_update(fs_entry)?;
    }

    Ok(())
}

/// Orchestrates a full recursive scan: emits `Started`, streams `Progress`
/// events per entry via `on_event`, then emits `Finished`.
///
/// Runs the synchronous walk on a blocking thread so the async executor
/// is free during the scan.
///
/// # Errors
///
/// Returns `Err(FsError::ChannelClosed)` if any channel send fails.
/// Returns `Err(FsError::TaskPanic)` if the blocking task panics.
/// Any other IO error from the walk is propagated unchanged.
pub async fn do_scan(
    path: String,
    scan_settings: ScanSettings,
    on_event: &Channel<ScanEvent>,
) -> Result<(), FsError> {
    on_event
        .send(ScanEvent::Started {})
        .map_err(|_| FsError::ChannelClosed)?;

    let channel = on_event.clone();
    tauri::async_runtime::spawn_blocking(move || {
        scan_path(path, scan_settings, |entry| {
            channel
                .send(ScanEvent::Progress { entry })
                .map_err(|_| FsError::ChannelClosed)
        })
    })
    .await
    .map_err(|_| FsError::TaskPanic)??;

    on_event
        .send(ScanEvent::Finished {})
        .map_err(|_| FsError::ChannelClosed)?;

    Ok(())
}
