use std::collections::HashMap;
use std::ops::Range;

/// A property definition for use with the `Fuseable` trait.
///
/// [`FuseProperty`] defines a field name and its associated weight for fuzzy searching.
/// The weight determines how much influence this field has on the overall search score.
///
/// # Examples
///
/// ```no_run
/// use fuse_lib::{config::Fuse, fuseable::Fuseable, types::FuseProperty};
///
/// struct Book<'a> {
///     title: &'a str,
///     author: &'a str,
/// }
///
/// impl Fuseable for Book<'_> {
///     fn properties(&self) -> Vec<FuseProperty> {
///         vec![
///             FuseProperty { value: String::from("title"), weight: 0.3 },
///             FuseProperty { value: String::from("author"), weight: 0.7 },
///         ]
///     }
///     
///     fn lookup(&self, key: &str) -> Option<&str> {
///         match key {
///             "title" => Some(self.title),
///             "author" => Some(self.author),
///             _ => None
///         }
///     }
/// }
/// ```
pub struct FuseProperty {
    /// The name of the field to be included in the search.
    pub value: String,
    /// The weight assigned to this field in the search algorithm.
    ///
    /// Higher weights give the field more influence on the final score.
    /// Typical values range from 0.0 to 1.0.
    pub weight: f64,
}

impl FuseProperty {
    /// Creates a new [`FuseProperty`] with the specified field name and weight 1.0.
    ///
    /// # Arguments
    ///
    /// * `value` - The name of the field to search
    ///
    /// # Examples
    ///
    /// ```
    /// # use fuse_lib::types::FuseProperty;
    /// let property = FuseProperty::init("title");
    /// assert_eq!(property.value, "title");
    /// assert_eq!(property.weight, 1.0);
    /// ```
    pub fn init(value: &str) -> Self {
        Self {
            value: String::from(value),
            weight: 1.0,
        }
    }

    /// Creates a new [`FuseProperty`] with the specified field name and weight.
    ///
    /// # Arguments
    ///
    /// * `value` - The name of the field to search
    /// * `weight` - The weight to assign to this field (typically 0.0 to 1.0)
    ///
    /// # Examples
    ///
    /// ```
    /// # use fuse_lib::types::FuseProperty;
    /// let property = FuseProperty::init_with_weight("author", 0.7);
    /// assert_eq!(property.value, "author");
    /// assert_eq!(property.weight, 0.7);
    /// ```
    #[allow(dead_code)]
    pub fn init_with_weight(value: &str, weight: f64) -> Self {
        Self {
            value: String::from(value),
            weight,
        }
    }
}

/// A compiled search pattern containing metadata for efficient searching.
///
/// `Pattern` stores the preprocessed search string along with precomputed
/// data structures that optimize the fuzzy search algorithm. This type is
/// typically created by calling [`Fuse::create_pattern`].
///
/// # Examples
///
/// ```no_run
/// use fuse_lib::config::Fuse;
/// let fuse = Fuse::default();
/// let pattern = fuse.create_pattern("Hello").unwrap();
/// ```
///
/// [`Fuse::create_pattern`]: crate::config::Fuse::create_pattern
pub struct Pattern {
    /// The processed search text (may be case-normalized or truncated).
    pub text: String,
    /// The length of the pattern text in bytes.
    pub len: usize,
    /// Bitmask used for efficient pattern matching.
    pub mask: u64,
    /// Character-to-bitmask mapping for the Bitap algorithm.
    pub alphabet: HashMap<u8, u64>,
}

/// The result of searching for a pattern in a list of strings.
///
/// This type is returned by methods that search through collections of strings
/// and need to track which item in the collection matched.
#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub struct SearchResult {
    /// The index of the matching item in the original collection.
    pub index: usize,
    /// The search score (0.0 = perfect match, 1.0 = no match).
    pub score: f64,
    /// Character ranges that matched the search pattern.
    ///
    /// Useful for highlighting matched portions in the UI.
    pub ranges: Vec<Range<usize>>,
}

/// The result of searching for a pattern in a single string.
///
/// Contains the match score and the ranges of characters that matched
/// the search pattern.
#[derive(Debug, PartialEq)]
pub struct ScoreResult {
    /// The search score (0.0 = perfect match, 1.0 = no match).
    pub score: f64,
    /// Character ranges that matched the search pattern.
    ///
    /// Each range represents a contiguous sequence of matched characters,
    /// useful for highlighting matches in user interfaces.
    pub ranges: Vec<Range<usize>>,
}

/// The result of searching a single field within a `Fuseable` object.
///
/// This represents the match result for one property of a struct that
/// implements the `Fuseable` trait.
#[derive(Debug, PartialEq)]
pub struct FResult {
    /// The name of the field that was searched.
    pub value: String,
    /// The search score for this field (0.0 = perfect match, 1.0 = no match).
    pub score: f64,
    /// Character ranges that matched the search pattern in this field.
    pub ranges: Vec<Range<usize>>,
}

/// The result of searching within a collection of `Fuseable` objects.
///
/// This type aggregates the search results from all searchable fields
/// of a single object in the collection.
#[derive(Debug, PartialEq)]
pub struct FuseableSearchResult {
    /// The index of the matching object in the original collection.
    pub index: usize,
    /// The overall search score for this object.
    ///
    /// This is typically computed by averaging or otherwise combining
    /// the scores from all searchable fields.
    pub score: f64,
    /// The search results for each individual field that was searched.
    pub results: Vec<FResult>,
}
