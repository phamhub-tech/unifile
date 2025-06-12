use crate::{api::ApiResponse, api_response, fs::types::Drive};

#[tauri::command]
pub fn get_drives() -> ApiResponse<Vec<Drive>> {
    let drives = sysinfo::Disks::new_with_refreshed_list()
        .iter()
        .map(|disk| Drive::from_disk(disk))
        .collect();

    api_response!(drives)
}
