use tauri::ipc::Channel;
use tauri::State;

use crate::api::ApiResponse;
use crate::fs::models::drive::Drive;
use crate::fs::models::entry::FSEntry;
use crate::fs::models::scan::ScanEvent;
use crate::settings::models::AppSettingsManager;

use super::controller;

/// Returns all mounted drives detected on the system.
///
/// This command is infallible — if the disk list cannot be retrieved,
/// `sysinfo` returns an empty list rather than an error.
#[tauri::command]
pub fn get_drives() -> ApiResponse<Vec<Drive>> {
    ApiResponse::ok(controller::get_drives())
}

/// Returns the immediate children of `path` (depth = 1).
///
/// Permission-denied entries are silently skipped. Any other IO error
/// stops enumeration early and is returned as an error response.
#[tauri::command]
pub fn get_entries(path: String) -> ApiResponse<Option<Vec<FSEntry>>> {
    controller::get_entries(&path).into()
}

/// Recursively scans `path` for all filesystem entries and streams each one
/// to the frontend via `on_event`.
///
/// Emits three event types in order:
/// - `started`  — scan has begun
/// - `progress` — one entry per file/folder discovered
/// - `finished` — scan completed successfully
///
/// If an error occurs the command returns an error `ApiResponse` and
/// `finished` is **not** emitted, so the frontend can use the absence of
/// `finished` as a signal that something went wrong.
///
/// Returns `Result` as required by Tauri for async commands that accept
/// borrowed state parameters.
#[tauri::command]
pub async fn scan_path(
    path: String,
    settings_manager: State<'_, AppSettingsManager>,
    on_event: Channel<ScanEvent>,
) -> Result<ApiResponse<Option<()>>, String> {
    // B1 fix: clone the settings before the MutexGuard is dropped so the
    // mutex is released immediately, not held for the entire scan.
    let scan_settings = match settings_manager.settings.lock() {
        Ok(guard) => guard.scan.clone(),
        Err(_) => return Ok(ApiResponse::err("Failed to acquire settings lock")),
    };

    Ok(controller::do_scan(path, scan_settings, &on_event)
        .await
        .into())
}
