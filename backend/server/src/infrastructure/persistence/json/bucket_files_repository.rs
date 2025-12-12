use crate::domain::files::entities::file_info::FileInfoRef;

const SHINNKU_BUCKET_JSON: &str = include_str!("../../../../../data/shinnku_bucket_files.json");
const GALGAME0_BUCKET_JSON: &str = include_str!("../../../../../data/galgame0_bucket_files.json");

lazy_static::lazy_static! {
    pub static ref SHINNKU_FILES: Vec<FileInfoRef> = serde_json::from_str(SHINNKU_BUCKET_JSON)
        .expect("Failed to parse shinnku bucket files");

    pub static ref GALGAME0_FILES: Vec<FileInfoRef> = serde_json::from_str(GALGAME0_BUCKET_JSON)
        .expect("Failed to parse galgame0 bucket files");
}

pub fn filter_galgame0_files(files: &[FileInfoRef], prefix: &str) -> Vec<FileInfoRef> {
    files
        .iter()
        .filter(|v| v.file_path.starts_with(prefix))
        .cloned()
        .collect()
}
