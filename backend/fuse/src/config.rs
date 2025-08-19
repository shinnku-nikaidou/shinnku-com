/// A fuzzy search engine with configurable parameters.
///
/// `Fuse` provides fuzzy string matching capabilities with customizable search behavior.
/// It can be configured to control search location, distance tolerance, matching threshold,
/// case sensitivity, and tokenization options.
///
/// # Examples
///
/// Creating a `Fuse` instance with default settings:
/// ```no_run
/// # use fuse_lib::config::Fuse;
/// let fuse = Fuse::default();
/// ```
///
/// Creating a `Fuse` instance with custom configuration:
/// ```no_run
/// # use fuse_lib::config::Fuse;
/// let fuse = Fuse {
///     location: 0,
///     distance: 100,
///     threshold: 0.6,
///     max_pattern_length: 32,
///     is_case_sensitive: false,
///     tokenize: false,
/// };
/// ```
///
/// Using the builder pattern:
/// ```no_run
/// # use fuse_lib::config::Fuse;
/// let fuse = Fuse::builder()
///     .threshold(0.4)
///     .case_sensitive(true)
///     .distance(200)
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct Fuse {
    /// The starting position for pattern matching (0-based index).
    pub location: i32,
    /// Maximum distance to search away from the `location`.
    pub distance: i32,
    /// Score threshold for search results.
    ///
    /// - `0.0` represents a perfect match
    /// - `1.0` represents a complete mismatch
    /// - Values closer to `0.0` are more strict
    pub threshold: f64,
    /// Maximum allowed length for search patterns.
    pub max_pattern_length: i32,
    /// Whether to perform case-sensitive matching.
    ///
    /// When `true`, 'A' and 'a' are treated as different characters.
    /// When `false`, the search is case-insensitive.
    pub is_case_sensitive: bool,
    /// Whether to tokenize search patterns by whitespace.
    ///
    /// When `true`, the pattern is split into individual words
    /// and each word is searched separately.
    pub tokenize: bool,
}

impl std::default::Default for Fuse {
    fn default() -> Self {
        Self {
            location: 0,
            distance: 100,
            threshold: 0.6,
            max_pattern_length: 32,
            is_case_sensitive: false,
            tokenize: false,
        }
    }
}

impl Fuse {
    /// Creates a new `Fuse` instance with the specified parameters.
    ///
    /// # Arguments
    ///
    /// * `location` - Starting position for pattern matching
    /// * `distance` - Maximum search distance from location
    /// * `threshold` - Score threshold (0.0 = perfect match, 1.0 = no match)
    /// * `max_pattern_length` - Maximum allowed pattern length
    /// * `is_case_sensitive` - Whether matching should be case-sensitive
    /// * `tokenize` - Whether to split patterns into tokens
    pub const fn new(
        location: i32,
        distance: i32,
        threshold: f64,
        max_pattern_length: i32,
        is_case_sensitive: bool,
        tokenize: bool,
    ) -> Self {
        Self {
            location,
            distance,
            threshold,
            max_pattern_length,
            is_case_sensitive,
            tokenize,
        }
    }

    /// Creates a new `FuseBuilder` with default settings.
    ///
    /// This is the recommended way to create a `Fuse` instance with custom configuration.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use fuse_lib::config::Fuse;
    /// let fuse = Fuse::builder()
    ///     .threshold(0.3)
    ///     .case_sensitive(true)
    ///     .build();
    /// ```
    pub const fn builder() -> FuseBuilder {
        FuseBuilder::new()
    }
}

/// Builder for creating `Fuse` instances with a fluent API.
///
/// `FuseBuilder` provides a convenient way to construct `Fuse` instances
/// with custom configuration while maintaining type safety and providing
/// sensible defaults.
///
/// # Examples
///
/// ```no_run
/// # use fuse_lib::config::Fuse;
/// let fuse = Fuse::builder()
///     .location(10)
///     .distance(50)
///     .threshold(0.4)
///     .case_sensitive(true)
///     .tokenize(false)
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct FuseBuilder {
    fuse: Fuse,
}

impl FuseBuilder {
    /// Creates a new `FuseBuilder` with default settings.
    pub const fn new() -> Self {
        Self {
            fuse: Fuse {
                location: 0,
                distance: 100,
                threshold: 0.6,
                max_pattern_length: 32,
                is_case_sensitive: false,
                tokenize: false,
            },
        }
    }

    /// Sets the starting location for pattern matching.
    ///
    /// # Arguments
    ///
    /// * `location` - The 0-based index where pattern matching should begin
    pub const fn location(mut self, location: i32) -> Self {
        self.fuse.location = location;
        self
    }

    /// Sets the maximum distance to search away from the location.
    ///
    /// # Arguments
    ///
    /// * `distance` - Maximum distance from the location to search
    pub const fn distance(mut self, distance: i32) -> Self {
        self.fuse.distance = distance;
        self
    }

    /// Sets the score threshold for accepting matches.
    ///
    /// # Arguments
    ///
    /// * `threshold` - Score threshold where 0.0 is a perfect match and 1.0 is no match
    pub const fn threshold(mut self, threshold: f64) -> Self {
        self.fuse.threshold = threshold;
        self
    }

    /// Sets the maximum allowed pattern length.
    ///
    /// # Arguments
    ///
    /// * `max_pattern_length` - Maximum number of characters allowed in search patterns
    pub const fn max_pattern_length(mut self, max_pattern_length: i32) -> Self {
        self.fuse.max_pattern_length = max_pattern_length;
        self
    }

    /// Sets whether matching should be case-sensitive.
    ///
    /// # Arguments
    ///
    /// * `is_case_sensitive` - `true` for case-sensitive matching, `false` for case-insensitive
    pub const fn case_sensitive(mut self, is_case_sensitive: bool) -> Self {
        self.fuse.is_case_sensitive = is_case_sensitive;
        self
    }

    /// Sets whether to tokenize search patterns by whitespace.
    ///
    /// # Arguments
    ///
    /// * `tokenize` - `true` to split patterns into words, `false` to search as a single pattern
    pub const fn tokenize(mut self, tokenize: bool) -> Self {
        self.fuse.tokenize = tokenize;
        self
    }

    /// Builds and returns the configured `Fuse` instance.
    pub const fn build(self) -> Fuse {
        self.fuse
    }
}

impl Default for FuseBuilder {
    fn default() -> Self {
        Self::new()
    }
}
