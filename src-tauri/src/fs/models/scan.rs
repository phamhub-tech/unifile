use serde::Serialize;

#[derive(Clone, Serialize)]
#[serde(tag="event", rename_all = "camelCase", content = "data")]
pub enum ScanEvent {
  Started {},
  Progress {
    entry: super::entry::FSEntry,
  },
  Finished {},
}