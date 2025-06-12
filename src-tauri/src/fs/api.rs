use crate::api::{ApiError, ApiResponse};
use crate::api_response;
use crate::fs::models::{drive::Drive, entry::FSEntry};

#[tauri::command]
pub fn get_drives() -> Result<ApiResponse<Vec<Drive>>, ApiError> {
    let drives = sysinfo::Disks::new_with_refreshed_list()
        .iter()
        .map(|disk| Drive::from_disk(disk))
        .collect();

    Ok(api_response!(drives))
}

#[tauri::command]
pub fn get_entries() -> Result<ApiResponse<Vec<FSEntry>>, ApiError> {
    Ok(api_response!(vec![]))
}
