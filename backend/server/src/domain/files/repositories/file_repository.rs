use crate::domain::files::entities::file_info::BucketFiles;
use anyhow::Result;

/// Repository trait for bucket files data access
pub trait BucketFilesRepository {
    /// Load Shinnku bucket files
    ///
    /// # Errors
    ///
    /// Returns an error if data loading fails
    fn load_shinnku_files(&self) -> Result<BucketFiles>;

    /// Load Galgame0 bucket files
    ///
    /// # Errors
    ///
    /// Returns an error if data loading fails
    fn load_galgame0_files(&self) -> Result<BucketFiles>;

    /// Filter files by path prefix
    fn filter_galgame0_files(&self, files: &BucketFiles, prefix: &str) -> BucketFiles;
}
