use crate::{api::{ApiError, ApiResponse}, api_response, fs::types::Drive};

#[tauri::command]
pub fn get_drives() -> Result<ApiResponse<Vec<Drive>>, ApiError> {
    let drives = sysinfo::Disks::new_with_refreshed_list()
        .iter()
        .map(|disk| Drive::from_disk(disk))
        .collect();

    Ok(api_response!(drives))
}
