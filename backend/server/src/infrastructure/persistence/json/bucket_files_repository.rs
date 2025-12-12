use crate::domain::files::entities::file_info::BucketFiles;

const SHINNKU_BUCKET_JSON: &str = include_str!("../../../../../data/shinnku_bucket_files.json");
const GALGAME0_BUCKET_JSON: &str = include_str!("../../../../../data/galgame0_bucket_files.json");

lazy_static::lazy_static! {
    pub static ref SHINNKU_FILES: BucketFiles = serde_json::from_str(SHINNKU_BUCKET_JSON).unwrap();
    pub static ref GALGAME0_FILES: BucketFiles = serde_json::from_str(GALGAME0_BUCKET_JSON).unwrap();
}

pub fn filter_galgame0_files(files: &BucketFiles, prefix: &str) -> BucketFiles {
    files
        .iter()
        .filter(|v| v.file_path.starts_with(prefix))
        .cloned()
        .collect()
}

