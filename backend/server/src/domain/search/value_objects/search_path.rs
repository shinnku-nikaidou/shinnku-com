/// Value object for search path normalization
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchPath(String);

impl SearchPath {
    /// Create a new SearchPath with normalization
    pub fn new(path: &str) -> Self {
        Self(Self::normalize(path))
    }

    // /// Get the normalized path string
    // pub fn as_str(&self) -> &str {
    //     &self.0
    // }

    /// Normalize file path for search indexing by removing common prefix
    fn normalize(file_path: &str) -> String {
        const PREFIX: &str = "合集系列/";
        file_path
            .strip_prefix(PREFIX)
            .unwrap_or(file_path)
            .to_string()
    }
}

impl From<&str> for SearchPath {
    fn from(path: &str) -> Self {
        Self::new(path)
    }
}

impl From<String> for SearchPath {
    fn from(path: String) -> Self {
        Self::new(&path)
    }
}

impl std::fmt::Display for SearchPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
