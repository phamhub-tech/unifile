use ignore::{overrides::OverrideBuilder, WalkBuilder};
use tracing::{debug, instrument, trace, warn};

use crate::fs::error::FsError;
use crate::fs::models::drive::Drive;
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
                warn!(error = %e, "Walk error, skipping entry");
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
                warn!(%path, "Permission denied, skipping entry");
                continue;
            }
            Err(FsError::NotFound { path }) => {
                warn!(%path, "Entry not found (removed during walk), skipping");
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
#[instrument(skip(on_update))]
pub fn scan<F>(path: String, scan_settings: ScanSettings, on_update: F) -> Result<(), FsError>
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
        override_builder.add(&format!("!{pattern}"))?;
    }
    let overrides = override_builder.build()?;
    builder.overrides(overrides);

    debug!(?scan_settings, "Walk starting");
    let mut count: u64 = 0;
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
                warn!(%path, "Permission denied, skipping entry");
                continue;
            }
            Err(FsError::NotFound { path }) => {
                warn!(%path, "Entry not found (removed during scan), skipping");
                continue;
            }
            // Fatal: stop the scan and report to the caller.
            Err(e) => return Err(e),
        };

        trace!(path = %fs_entry.path, kind = ?fs_entry.entry_type, "Entry discovered");
        count += 1;
        on_update(fs_entry)?;
    }

    debug!(count, "Walk complete");
    Ok(())
}
