use std::path::PathBuf;

use serde::Serialize;
use sysinfo::{Disk, DiskKind};

#[derive(Debug, Serialize)]
pub enum DriveType {
    #[serde(rename = "hdd")]
    Hdd,

    #[serde(rename = "ssd")]
    Ssd,

    #[serde(rename = "Unknown")]
    Unknown,
}

#[derive(Debug, Serialize)]
pub struct Drive {
    name: String,
    total_bytes: u64,
    used_bytes: u64,
    available_bytes: u64,
    mount_point: PathBuf,
    is_removable: bool,
    drive_type: DriveType,
}
impl Drive {
    pub fn from_disk(disk: &Disk) -> Self {
        let total_bytes = disk.total_space();
        let available_bytes = disk.available_space();
        let used_bytes = total_bytes - available_bytes;

        let name = disk.name().to_str().unwrap_or("Local Disk").to_string();
        let mount_point = disk.mount_point().to_path_buf();
        let drive_type = match disk.kind() {
            DiskKind::HDD => DriveType::Hdd,
            DiskKind::SSD => DriveType::Ssd,
            DiskKind::Unknown(_) => DriveType::Unknown,
        };

        Self {
            name,
            total_bytes,
            used_bytes,
            available_bytes,
            mount_point,
            is_removable: disk.is_removable(),
            drive_type,
        }
    }
}