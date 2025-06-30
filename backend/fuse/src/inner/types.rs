use std::collections::HashMap;
use std::ops::Range;

/// Defines the fuseproperty object to be returned as part of the list
/// returned by properties() implemented by the Fuseable trait.
/// # Examples:
/// Basic Usage:
/// ```no_run
/// use crate::fuse::lib::{ Fuse, Fuseable, FuseProperty };
/// struct Book<'a> {
///     title: &'a str,
///     author: &'a str,
/// }
///
/// impl Fuseable for Book<'_>{
///     fn properties(&self) -> Vec<FuseProperty> {
///         return vec!(
///             FuseProperty{value: String::from("title"), weight: 0.3},
///             FuseProperty{value: String::from("author"), weight: 0.7},
///         )
///     }
///     fn lookup(&self, key: &str) -> Option<&str> {
///         return match key {
///             "title" => Some(self.title),
///             "author" => Some(self.author),
///             _ => None
///         }
///     }
/// }
/// ```
pub struct FuseProperty {
    /// The name of the field with an associated weight in the search.
    pub value: String,
    /// The weight associated with the specified field.
    pub weight: f64,
}

impl FuseProperty {
    /// create a fuse property with weight 1.0 and a string reference.
    pub fn init(value: &str) -> Self {
        Self {
            value: String::from(value),
            weight: 1.0,
        }
    }
    /// create a fuse property with a specified weight and string reference.
    #[allow(dead_code)]
    pub fn init_with_weight(value: &str, weight: f64) -> Self {
        Self {
            value: String::from(value),
            weight,
        }
    }
}

/// A datatype to store the pattern's text, its length, a mask
/// and a hashmap against each alphabet in the text.
/// Always use fuse.create_pattern("search string") to create a pattern
/// # Examples:
/// Basic usage:
/// ```no_run
/// use crate::fuse::lib::{ Fuse };
/// let fuse = Fuse::default();
/// let pattern = fuse.create_pattern("Hello");
/// ```
pub struct Pattern {
    pub text: String,
    pub len: usize,
    pub mask: u64,
    pub alphabet: HashMap<u8, u64>,
}

/// Return type for performing a search on a list of strings
#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub struct SearchResult {
    /// corresponding index of the search result in the original list
    pub index: usize,
    /// Search rating of the search result, 0.0 is a perfect match 1.0 is a perfect mismatch
    pub score: f64,
    /// Ranges of matches in the search query, useful if you want to hightlight matches.
    pub ranges: Vec<Range<usize>>,
}

/// Return type for performing a search on a single string.
#[derive(Debug, PartialEq)]
pub struct ScoreResult {
    /// Search rating of the search result, 0.0 is a perfect match 1.0 is a perfect mismatch
    pub score: f64,
    /// Ranges of matches in the search query, useful if you want to hightlight matches.
    pub ranges: Vec<Range<usize>>,
}

/// Return type for performing a search with a single fuseable property of struct
#[derive(Debug, PartialEq)]
pub struct FResult {
    /// The corresponding field name for this search result
    pub value: String,
    /// Search rating of the search result, 0.0 is a perfect match 1.0 is a perfect mismatch
    pub score: f64,
    /// Ranges of matches in the search query, useful if you want to hightlight matches.
    pub ranges: Vec<Range<usize>>,
}

/// Return type for performing a search over a list of Fuseable structs
#[derive(Debug, PartialEq)]
pub struct FuseableSearchResult {
    /// corresponding index of the search result in the original list
    pub index: usize,
    /// Search rating of the search result, 0.0 is a perfect match 1.0 is a perfect mismatch
    pub score: f64,
    /// Ranges of matches in the search query, useful if you want to hightlight matches.
    pub results: Vec<FResult>,
}
