use file_format::{FileFormat, Kind};
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub enum FSEntryType {
    #[serde(rename = "folder")]
    Folder,

    #[serde(rename = "file")]
    File,
}

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
    pub fn from_file_path(path: &str) -> Option<Self> {
        let file_kind = match FileFormat::from_file(&path) {
            Ok(kind) => kind.kind(),
            Err(_) => return Some(FileType::Other),
        };

        let file_type = match file_kind {
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

#[derive(Debug, Serialize)]
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
    // pub fn from_entry(entry: &walkdir::DirEntry) -> Result<Self, std::io::Error> {
    //     let entry_type = match entry.file_type().is_dir() {
    //         true => FSEntryType::Folder,
    //         false => FSEntryType::File,
    //     };
    //     let path = get_entry_path(&entry);

    //     let metadata = fs::metadata(&path)?;
    //     let created = match metadata.created() {
    //         Ok(time) => {
    //             let datetime: chrono::DateTime<chrono::Local> = time.into();
    //             Some(format!("{}", datetime))
    //         }
    //         Err(_) => None,
    //     };
    //     let modified = match metadata.modified() {
    //         Ok(time) => {
    //             let datetime: chrono::DateTime<chrono::Local> = time.into();
    //             Some(datetime.to_rfc3339())
    //         }
    //         Err(_) => None,
    //     };
    //     let file_type = FileType::from_file_path(&path);

    //     Ok(Self {
    //         name: get_entry_name(entry),
    //         path,
    //         size: metadata.len(),
    //         file_type,
    //         created,
    //         modified,
    //         entry_type,
    //     })
    // }
}
