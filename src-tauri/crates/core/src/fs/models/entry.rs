use std::{fs, io::Read, path::Path};

use file_format::{FileFormat, Kind};
use serde::Serialize;

use crate::fs::error::FsError;

/// Whether a filesystem entry is a file or a directory.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum FSEntryType {
    #[serde(rename = "folder")]
    Folder,

    #[serde(rename = "file")]
    File,
}

/// Broad category of a file's content.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize)]
pub enum FileType {
    #[serde(rename = "audio")]
    Audio,

    #[serde(rename = "app")]
    App,

    #[serde(rename = "archive")]
    Archive,

    #[serde(rename = "document")]
    Document,

    #[serde(rename = "ebook")]
    Ebook,

    #[serde(rename = "image")]
    Image,

    #[serde(rename = "video")]
    Video,

    #[default]
    #[serde(rename = "other")]
    Other,
}
impl FileType {
    /// Detects the [`FileType`] of the file at `path` by reading only its
    /// first 4096 bytes (enough to cover all common magic numbers).
    ///
    /// Reading a bounded prefix is far cheaper than `FileFormat::from_file`,
    /// which reads the entire file. Returns [`FileType::Other`] when the file
    /// cannot be opened or its format is unrecognised.
    pub fn from_file_path<P: AsRef<Path>>(path: &P) -> Option<Self> {
        let mut buf = [0u8; 4096];
        let n = match fs::File::open(path.as_ref()) {
            Ok(mut f) => f.read(&mut buf).unwrap_or(0),
            Err(_) => return Some(FileType::Other),
        };

        let file_type = match FileFormat::from_bytes(&buf[..n]).kind() {
            Kind::Executable => FileType::App,
            Kind::Audio => FileType::Audio,
            Kind::Document => FileType::Document,
            Kind::Image => FileType::Image,
            Kind::Video => FileType::Video,
            Kind::Archive => FileType::Archive,
            Kind::Ebook => FileType::Ebook,
            _ => FileType::Other,
        };

        Some(file_type)
    }
}

/// A single file or directory entry in the filesystem.
#[derive(Clone, Debug, Serialize)]
pub struct FSEntry {
    pub size: u64,
    pub file_type: Option<FileType>,
    pub entry_type: FSEntryType,
    pub name: String,
    pub path: String,
    created: Option<String>,
    pub modified: Option<String>,
}
impl FSEntry {
    /// Builds an [`FSEntry`] from a directory-walker entry.
    ///
    /// Converts IO errors into typed [`FsError`] variants so callers can
    /// distinguish recoverable errors (permission denied, not found) from
    /// unexpected failures without matching on [`std::io::ErrorKind`].
    pub fn from_entry(entry: &ignore::DirEntry) -> Result<Self, FsError> {
        let path = entry.path();
        let path_str = path.display().to_string();
        // Use the metadata cached by the walker instead of a second
        // `fs::metadata` syscall. Falls back to `fs::metadata` if the walker
        // did not cache it (e.g. the entry came from a non-caching source).
        let metadata = entry
            .metadata()
            .or_else(|_| fs::metadata(entry.path()))
            .map_err(|e| FsError::from_io(&path_str, e))?;

        let created = match metadata.created() {
            Ok(time) => {
                let datetime: chrono::DateTime<chrono::Local> = time.into();
                Some(format!("{}", datetime))
            }
            Err(_) => None,
        };
        let modified = match metadata.modified() {
            Ok(time) => {
                let datetime: chrono::DateTime<chrono::Local> = time.into();
                Some(datetime.to_rfc3339())
            }
            Err(_) => None,
        };

        let mut entry_type = FSEntryType::Folder;
        let mut file_type = None; // Folders don't have file types
        if entry
            .file_type()
            // file_type() is None only for stdin, which cannot occur during a walk.
            .unwrap_or_else(|| panic!("Entry has no file_type: {:?}", entry.path()))
            .is_file()
        {
            file_type = FileType::from_file_path(&path);
            entry_type = FSEntryType::File;
        }

        Ok(Self {
            name: entry.file_name().to_string_lossy().to_string(),
            path: path_str,
            size: metadata.len(),
            file_type,
            created,
            modified,
            entry_type,
        })
    }
}
