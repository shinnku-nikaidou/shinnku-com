use crate::models::BucketFiles;
use anyhow::Result;

/// Repository for handling bucket files data access
pub struct BucketFilesRepository;

impl BucketFilesRepository {
    pub fn new() -> Self {
        Self
    }

    /// Load Shinnku bucket files from embedded JSON
    ///
    /// # Errors
    ///
    /// Returns an error if JSON parsing fails
    pub fn load_shinnku_files(&self) -> Result<BucketFiles> {
        let raw = include_str!("../../../data/shinnku_bucket_files.json");
        Ok(serde_json::from_str(raw)?)
    }

    /// Load Galgame0 bucket files from embedded JSON
    ///
    /// # Errors
    ///
    /// Returns an error if JSON parsing fails
    pub fn load_galgame0_files(&self) -> Result<BucketFiles> {
        let raw = include_str!("../../../data/galgame0_bucket_files.json");
        Ok(serde_json::from_str(raw)?)
    }

    /// Filter galgame0 files to only include specific path prefix
    pub fn filter_galgame0_files(&self, files: &BucketFiles, prefix: &str) -> BucketFiles {
        files
            .iter()
            .filter(|v| v.file_path.starts_with(prefix))
            .cloned()
            .collect()
    }
}

impl Default for BucketFilesRepository {
    fn default() -> Self {
        Self::new()
    }
}
