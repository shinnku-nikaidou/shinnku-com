use super::config::Fuse;
use super::types::{FuseProperty, FuseableSearchResult};
use crate::types::{Pattern, ScoreResult};
use crate::utils;

/// A trait for objects that can be searched using fuzzy matching.
///
/// Types implementing `Fuseable` can be searched across multiple fields,
/// with each field having its own weight that influences the final score.
/// This is useful for searching complex data structures like database records,
/// configuration objects, or any structured data.
///
/// # Required Methods
///
/// * [`properties`] - Returns a list of searchable fields with their weights
/// * [`lookup`] - Returns the string value for a given field name
///
/// # Examples
///
/// ```no_run
/// use fuse_lib::config::Fuse;
/// use fuse_lib::fuseable::Fuseable;
/// use fuse_lib::types::FuseProperty;
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
///
/// [`properties`]: #tymethod.properties
/// [`lookup`]: #tymethod.lookup
pub trait Fuseable {
    /// Returns a list of searchable fields with their associated weights.
    ///
    /// Each `FuseProperty` should specify a field name that can be looked up
    /// using the `lookup` method, along with a weight that determines how much
    /// influence this field has on the overall search score.
    ///
    /// # Returns
    ///
    /// A vector of `FuseProperty` objects defining the searchable fields.
    fn properties(&self) -> Vec<FuseProperty>;

    /// Returns the string value for the specified field name.
    ///
    /// This method should return the actual text content that should be
    /// searched for the given field. Return `None` if the field doesn't
    /// exist or cannot be searched.
    ///
    /// # Arguments
    ///
    /// * `key` - The name of the field to look up
    ///
    /// # Returns
    ///
    /// The string content of the field, or `None` if the field is not found.
    fn lookup(&self, key: &str) -> Option<&str>;
}

impl Fuse {
    /// Searches for a text pattern across multiple fields in a collection of `Fuseable` objects.
    ///
    /// This method performs fuzzy searching across all searchable properties of each object
    /// in the collection. The results are sorted by relevance score, with the best matches
    /// appearing first.
    ///
    /// # Arguments
    ///
    /// * `text` - The search pattern to look for
    /// * `list` - A slice of objects implementing the `Fuseable` trait
    ///
    /// # Returns
    ///
    /// A vector of `FuseableSearchResult` objects, sorted by relevance (best matches first).
    /// Each result contains the object's index in the original collection, the overall score,
    /// and detailed results for each field that matched.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use fuse_lib::config::Fuse;
    /// # use fuse_lib::fuseable::Fuseable;
    /// # use fuse_lib::types::FuseProperty;
    /// #
    /// # struct Book<'a> {
    /// #    title: &'a str,
    /// #    author: &'a str,
    /// # }
    /// #
    /// # impl Fuseable for Book<'_> {
    /// #     fn properties(&self) -> Vec<FuseProperty> {
    /// #         vec![
    /// #             FuseProperty { value: String::from("title"), weight: 0.3 },
    /// #             FuseProperty { value: String::from("author"), weight: 0.7 },
    /// #         ]
    /// #     }
    /// #
    /// #     fn lookup(&self, key: &str) -> Option<&str> {
    /// #         match key {
    /// #             "title" => Some(self.title),
    /// #             "author" => Some(self.author),
    /// #             _ => None
    /// #         }
    /// #     }
    /// # }   
    /// let books = [
    ///     Book { author: "John X", title: "Old Man's War fiction" },
    ///     Book { author: "P.D. Mans", title: "Right Ho Jeeves" },
    /// ];
    ///
    /// let fuse = Fuse::default();
    /// let results = fuse.search_text_in_fuse_list("man", &books);
    /// ```
    pub fn search_text_in_fuse_list(
        &self,
        text: &str,
        list: &[impl Fuseable],
    ) -> Vec<FuseableSearchResult> {
        let pattern = self.create_pattern(text);

        let mut result: Vec<FuseableSearchResult> = list
            .iter()
            .enumerate()
            .filter_map(|(index, item)| self.search_fuseable_item(pattern.as_ref(), index, item))
            .collect();

        result.sort_unstable_by(|a, b| {
            a.score
                .partial_cmp(&b.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        result
    }

    /// Searches for a pattern in the given string.
    ///
    /// This method performs fuzzy string matching using the Bitap algorithm
    /// with the current configuration settings. It supports both tokenized
    /// and non-tokenized search modes.
    ///
    /// # Arguments
    ///
    /// * `pattern` - The compiled pattern to search for (created with [`create_pattern`])
    /// * `string` - The target string to search within
    ///
    /// # Returns
    ///
    /// Returns `Some(ScoreResult)` containing the match score and character ranges
    /// if a match is found, or `None` if no match meets the threshold criteria.
    ///
    /// The score ranges from 0.0 (perfect match) to 1.0 (no match).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use fuse_lib::config::Fuse;
    /// let fuse = Fuse::default();
    /// let pattern = fuse.create_pattern("some text");
    /// let result = fuse.search(pattern.as_ref(), "some string");
    /// ```
    ///
    /// [`Fuse::create_pattern`] : #method.create_pattern
    pub fn search(&self, pattern: Option<&Pattern>, string: &str) -> Option<ScoreResult> {
        let pattern = pattern?;

        if self.tokenize {
            let word_patterns = pattern
                .text
                .split_whitespace()
                .filter_map(|word| self.create_pattern(word));

            let full_pattern_result = self.search_util(pattern, string);

            let (length, results) = word_patterns.fold(
                (0, full_pattern_result),
                |(count, mut total_result), word_pattern| {
                    let mut result = self.search_util(&word_pattern, string);
                    total_result.score += result.score;
                    total_result.ranges.append(&mut result.ranges);
                    (count + 1, total_result)
                },
            );

            let averaged_result = ScoreResult {
                score: results.score / f64::from(length + 1),
                ranges: results.ranges,
            };

            // Use a more precise comparison for floating point
            if averaged_result.score >= 1.0 - f64::EPSILON {
                None
            } else {
                Some(averaged_result)
            }
        } else {
            let result = self.search_util(pattern, string);
            // Use a more precise comparison for floating point
            if result.score >= 1.0 - f64::EPSILON {
                None
            } else {
                Some(result)
            }
        }
    }

    /// Creates a pattern object from the input string.
    ///
    /// This method preprocesses the input string according to the current
    /// configuration settings (case sensitivity, maximum pattern length, etc.)
    /// and returns a `Pattern` object optimized for efficient searching.
    ///
    /// # Arguments
    ///
    /// * `string` - The text pattern to compile for searching
    ///
    /// # Returns
    ///
    /// Returns `Some(Pattern)` if the string is valid and non-empty,
    /// or `None` if the string is empty or invalid.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use fuse_lib::config::Fuse;
    /// let fuse = Fuse::default();
    /// let pattern = fuse.create_pattern("hello world").unwrap();
    /// ```
    pub fn create_pattern(&self, string: &str) -> Option<Pattern> {
        if string.is_empty() {
            return None;
        }

        let pattern_text = if self.is_case_sensitive {
            string.to_owned()
        } else {
            string.to_lowercase()
        };

        // Truncate pattern to max_pattern_length to prevent overflow
        // Use character-boundary-safe truncation for UTF-8 strings
        let max_len = self.max_pattern_length;
        let (truncated_text, truncated_len) = if pattern_text.len() > max_len {
            // Find the closest character boundary at or before max_len
            let boundary = pattern_text
                .char_indices()
                .map(|(i, _)| i)
                .take_while(|&i| i <= max_len)
                .last()
                .unwrap_or(0);

            if boundary == 0 {
                // If no valid boundary found, use empty string
                return None;
            }

            let truncated = &pattern_text[..boundary];
            (truncated.to_owned(), truncated.len())
        } else {
            (pattern_text.clone(), pattern_text.len())
        };

        let truncated_chars = truncated_text.as_bytes();
        let alphabet = utils::calculate_pattern_alphabet(truncated_chars);

        // Prevent bit shift overflow for very long patterns
        let mask = if truncated_len > 64 || truncated_len == 0 {
            0 // For very long patterns or empty patterns, use 0 as mask
        } else {
            1_u64
                .checked_shl(u32::try_from(truncated_len - 1).unwrap_or(0))
                .unwrap_or(0)
        };

        Some(Pattern {
            text: truncated_text,
            len: truncated_len,
            mask,
            alphabet,
        })
    }
}
