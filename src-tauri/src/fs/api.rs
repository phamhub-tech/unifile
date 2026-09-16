use tauri::ipc::Channel;
use tauri::State;
use tracing::{debug, error, info, instrument};

use unifile_core::fs;
use unifile_core::fs::{scan, Drive, FSEntry, FsError, FsResult, ScanEvent};

use unifile_core::settings::ScanSettings;

use crate::api::ApiResponse;
use crate::settings::models::AppSettingsManager;

/// Returns all mounted drives detected on the system.
///
/// This command is infallible — if the disk list cannot be retrieved,
/// `sysinfo` returns an empty list rather than an error.
#[tauri::command]
pub fn get_drives() -> ApiResponse<Vec<Drive>> {
    ApiResponse::ok(fs::get_drives())
}

/// Returns the immediate children of `path` (depth = 1).
///
/// Permission-denied entries are silently skipped. Any other IO error
/// stops enumeration early and is returned as an error response.
#[tauri::command]
pub fn get_entries(path: String) -> ApiResponse<Option<Vec<FSEntry>>> {
    fs::get_entries(&path).into()
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
#[instrument(skip(settings_manager, on_event))]
pub async fn scan_path(
    path: String,
    settings_manager: State<'_, AppSettingsManager>,
    on_event: Channel<ScanEvent>,
) -> Result<ApiResponse<Option<()>>, String> {
    let scan_settings = match settings_manager.settings.lock() {
        Ok(guard) => guard.scan.clone(),
        Err(_) => {
            error!("Failed to acquire settings lock");
            return Ok(ApiResponse::err("Failed to acquire settings lock"));
        }
    };

    debug!(?scan_settings, "Scan requested");
    Ok(do_scan(path, scan_settings, &on_event).await.into())
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
#[instrument(skip(scan_settings, on_event))]
pub async fn do_scan(
    path: String,
    scan_settings: ScanSettings,
    on_event: &Channel<ScanEvent>,
) -> FsResult<()> {
    info!("Scan started");

    on_event
        .send(ScanEvent::Started {})
        .map_err(|_| FsError::ChannelClosed)?;

    let channel = on_event.clone();
    tauri::async_runtime::spawn_blocking(move || {
        scan(path, scan_settings, |entry| {
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

    info!("Scan finished");
    Ok(())
}
