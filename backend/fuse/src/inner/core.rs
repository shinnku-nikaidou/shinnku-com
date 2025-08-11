use super::types::{FResult, FuseProperty, FuseableSearchResult, Pattern, ScoreResult};
use crate::utils;

/// Creates a new fuse object with given config settings
/// Use to create patterns and access the search methods.
/// Also implements a default method to quickly get a fuse
/// object ready with the default config.
/// # Examples:
/// Basic Usage:
/// ```no_run
/// # use crate::fuse::lib::{ Fuse };
/// let fuse = Fuse{
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
/// # use crate::fuse::lib::{ Fuse };
/// let fuse = Fuse::builder()
///     .threshold(0.4)
///     .case_sensitive(true)
///     .distance(200)
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct Fuse {
    /// location to starting looking for patterns
    pub location: i32,
    /// maximum distance to look away from the location
    pub distance: i32,
    /// threshold for the search algorithm to give up at, 0.0 is perfect match 1.0 is imperfect match
    pub threshold: f64,
    /// maximum allowed pattern length
    pub max_pattern_length: i32,
    /// check for lowercase and uppercase seperately
    pub is_case_sensitive: bool,
    /// tokenize search patterns
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
    /// Creates a new `Fuse` instance with specified parameters
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

    /// Creates a new `Fuse` builder with default settings
    pub const fn builder() -> FuseBuilder {
        FuseBuilder::new()
    }

    /// Creates a pattern object from input string.
    ///
    /// - Parameter string: A string from which to create the pattern object
    /// - Returns: A tuple containing pattern metadata
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
        let max_len = usize::try_from(self.max_pattern_length.max(0)).unwrap_or(0);
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

    #[allow(
        clippy::single_range_in_vec_init,
        clippy::cast_possible_truncation,
        clippy::cast_possible_wrap,
        clippy::cast_sign_loss
    )]
    fn search_util(&self, pattern: &Pattern, string: &str) -> ScoreResult {
        let string_to_search = if self.is_case_sensitive {
            string
        } else {
            // Only allocate when necessary
            &string.to_ascii_lowercase()
        };

        let string_chars = string_to_search.as_bytes();
        let text_length = string_to_search.len();

        // Exact match
        if pattern.text == string_to_search {
            return ScoreResult {
                score: 0.,
                ranges: vec![0..text_length],
            };
        }

        let location = self.location;
        let distance = self.distance;
        let mut threshold = self.threshold;
        let mut best_location = string_to_search.find(&pattern.text).unwrap_or(0_usize);
        let mut match_mask_arr = vec![0; text_length];

        // Helper function to safely find substring starting from a byte index
        let safe_find = |s: &str, start: usize, pattern: &str| -> Option<usize> {
            if start >= s.len() {
                return None;
            }
            // Find the next valid character boundary at or after start
            let mut boundary = start;
            while boundary < s.len() && !s.is_char_boundary(boundary) {
                boundary += 1;
            }
            if boundary >= s.len() {
                None
            } else {
                s[boundary..]
                    .find(pattern)
                    .map(|pos| boundary + pos - start)
            }
        };

        let mut index = safe_find(string_to_search, best_location, &pattern.text);
        let mut score;

        while let Some(offset) = index {
            let i = best_location + offset;
            score = utils::calculate_score(pattern.len, 0, i as i32, location, distance);
            threshold = threshold.min(score);
            best_location = i + pattern.len;
            index = safe_find(string_to_search, best_location, &pattern.text);

            for idx in 0..pattern.len {
                if i + idx < match_mask_arr.len() {
                    match_mask_arr[i + idx] = 1;
                }
            }
        }

        score = 1.;
        let mut bin_max = pattern.len + text_length;
        let mut last_bit_arr = vec![];

        let text_count = string_chars.len();

        for i in 0..pattern.len {
            let mut bin_min = 0;
            let mut bin_mid = bin_max;
            while bin_min < bin_mid {
                if utils::calculate_score(
                    pattern.len,
                    i as i32,
                    location,
                    location + bin_mid as i32,
                    distance,
                ) <= threshold
                {
                    bin_min = bin_mid;
                } else {
                    bin_max = bin_mid;
                }
                bin_mid = ((bin_max - bin_min) / 2) + bin_min;
            }
            bin_max = bin_mid;

            let start = if location >= 0 {
                1_usize.max(
                    (location as usize)
                        .saturating_sub(bin_mid)
                        .saturating_add(1),
                )
            } else {
                1_usize
            };
            let finish = if location >= 0 {
                text_length
                    .min((location as usize).saturating_add(bin_mid))
                    .saturating_add(pattern.len)
            } else {
                text_length.min(bin_mid).saturating_add(pattern.len)
            };

            let mut bit_arr = vec![0; finish + 2];

            // Prevent shift overflow - if i >= 64, use max value
            bit_arr[finish + 1] = if i < 64 { (1 << i) - 1 } else { u64::MAX };

            if start > finish {
                continue;
            }

            let mut current_location_index: usize = 0;
            for current_location in (start..=finish).rev() {
                let char_match: u64 = if current_location.saturating_sub(1) < text_count {
                    current_location_index = current_location_index
                        .checked_sub(1)
                        .unwrap_or(current_location.saturating_sub(1));
                    string_to_search
                        .as_bytes()
                        .get(current_location_index)
                        .and_then(|c| pattern.alphabet.get(c))
                        .copied()
                        .unwrap_or(0)
                } else {
                    0
                };

                if char_match != 0
                    && let Some(mask_item) =
                        match_mask_arr.get_mut(current_location.saturating_sub(1))
                {
                    *mask_item = 1;
                }

                bit_arr[current_location] = ((bit_arr[current_location + 1] << 1) | 1) & char_match;
                if i > 0 {
                    bit_arr[current_location] |= (((last_bit_arr[current_location + 1]
                        | last_bit_arr[current_location])
                        << 1_u64)
                        | 1)
                        | last_bit_arr[current_location + 1];
                };

                if (bit_arr[current_location] & pattern.mask) != 0 {
                    score = utils::calculate_score(
                        pattern.len,
                        i as i32,
                        location,
                        current_location.saturating_sub(1) as i32,
                        distance,
                    );

                    if score <= threshold {
                        threshold = score;
                        best_location = current_location.saturating_sub(1);

                        if best_location as i32 <= location {
                            break;
                        };
                    }
                }
            }
            if utils::calculate_score(pattern.len, i as i32 + 1, location, location, distance)
                > threshold
            {
                break;
            }

            last_bit_arr = bit_arr.clone();
        }

        ScoreResult {
            score,
            ranges: utils::find_ranges(&match_mask_arr).unwrap_or_default(),
        }
    }

    /// Searches for a pattern in a given string.
    /// - Parameters:
    ///   - pattern: The pattern to search for. This is created by calling `createPattern`
    ///   - string: The string in which to search for the pattern
    /// - Returns: Some(ScoreResult) if a match is found containing a `score` between `0.0` (exact match) and `1` (not a match), and `ranges` of the matched characters. If no match is found or if search pattern was empty will return None.
    /// # Example:
    /// ```no_run
    /// use crate::fuse::lib::{ Fuse };
    /// let fuse = Fuse::default();
    /// let pattern = fuse.create_pattern("some text");
    /// fuse.search(pattern.as_ref(), "some string");
    /// ```
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
}

/// Implementable trait for user defined structs, requires two methods to me implemented.
/// A properties method that should return a list of FuseProperties.
/// and a lookup method which should return the value of field, provided the field name.
/// # Examples:
/// Usage:
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
pub trait Fuseable {
    /// Returns a list of FuseProperty that contains the field name and its corresponding weight
    fn properties(&self) -> Vec<FuseProperty>;
    /// Provided a field name as argument, returns the value of the field. eg book.loopkup("author") === book.author
    fn lookup(&self, key: &str) -> Option<&str>;
}

impl Fuse {
    /// Searches for a text pattern in an array of `Fuseable` objects.
    /// - Parameters:
    ///   - text: The pattern string to search for
    ///   - list: A list of `Fuseable` objects, i.e. structs implementing the Fuseable trait in which to search
    /// - Returns: A list of `FuseableSearchResult` objects
    ///   Each `Fuseable` object contains a `properties` method which returns `FuseProperty` array. Each `FuseProperty` is a struct containing a `value` (the name of the field which should be included in the search), and a `weight` (how much "weight" to assign to the score)
    ///
    /// # Example
    /// ```no_run
    /// # use crate::fuse::lib::{ Fuse, Fuseable, FuseProperty };
    ///
    /// struct Book<'a> {
    ///    title: &'a str,
    ///    author: &'a str,
    /// }
    ///
    /// impl Fuseable for Book<'_>{
    ///     fn properties(&self) -> Vec<FuseProperty> {
    ///         return vec!(
    ///             FuseProperty{value: String::from("title"), weight: 0.3},
    ///             FuseProperty{value: String::from("author"), weight: 0.7},
    ///         )
    ///     }
    ///
    ///     fn lookup(&self, key: &str) -> Option<&str> {
    ///         return match key {
    ///             "title" => Some(self.title),
    ///             "author" => Some(self.author),
    ///             _ => None
    ///         }
    ///     }
    /// }   
    /// let books = [
    ///     Book{author: "John X", title: "Old Man's War fiction"},
    ///     Book{author: "P.D. Mans", title: "Right Ho Jeeves"},
    /// ];
    ///
    /// let fuse = Fuse::default();
    /// let results = fuse.search_text_in_fuse_list("man", &books);
    ///
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
            .filter_map(|(index, item)| {
                let mut total_score = 0.0;
                let mut property_results = Vec::new();

                for property in &item.properties() {
                    let Some(value) = item.lookup(&property.value) else {
                        // Skip properties that don't exist rather than panicking
                        continue;
                    };

                    if let Some(search_result) = self.search(pattern.as_ref(), value) {
                        let weight = if (property.weight - 1.0).abs() < f64::EPSILON {
                            1.0
                        } else {
                            1.0 - property.weight
                        };

                        let score =
                            if search_result.score == 0.0 && (weight - 1.0).abs() < f64::EPSILON {
                                0.001
                            } else {
                                search_result.score * weight
                            };

                        total_score += score;
                        property_results.push(FResult {
                            value: property.value.clone(),
                            score,
                            ranges: search_result.ranges,
                        });
                    }
                }

                if property_results.is_empty() {
                    None
                } else {
                    let count = property_results.len() as f64;
                    Some(FuseableSearchResult {
                        index,
                        score: total_score / count,
                        results: property_results,
                    })
                }
            })
            .collect();

        result.sort_unstable_by(|a, b| {
            a.score
                .partial_cmp(&b.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        result
    }
}

/// Builder for creating `Fuse` instances with a fluent API
#[derive(Debug, Clone)]
pub struct FuseBuilder {
    fuse: Fuse,
}

impl FuseBuilder {
    /// Creates a new `FuseBuilder` with default settings
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

    /// Sets the location to start looking for patterns
    pub const fn location(mut self, location: i32) -> Self {
        self.fuse.location = location;
        self
    }

    /// Sets the maximum distance to look away from the location
    pub const fn distance(mut self, distance: i32) -> Self {
        self.fuse.distance = distance;
        self
    }

    /// Sets the threshold for the search algorithm
    /// 0.0 is perfect match, 1.0 is imperfect match
    pub const fn threshold(mut self, threshold: f64) -> Self {
        self.fuse.threshold = threshold;
        self
    }

    /// Sets the maximum allowed pattern length
    pub const fn max_pattern_length(mut self, max_pattern_length: i32) -> Self {
        self.fuse.max_pattern_length = max_pattern_length;
        self
    }

    /// Sets whether to check for lowercase and uppercase separately
    pub const fn case_sensitive(mut self, is_case_sensitive: bool) -> Self {
        self.fuse.is_case_sensitive = is_case_sensitive;
        self
    }

    /// Sets whether to tokenize search patterns
    pub const fn tokenize(mut self, tokenize: bool) -> Self {
        self.fuse.tokenize = tokenize;
        self
    }

    /// Builds the `Fuse` instance
    pub const fn build(self) -> Fuse {
        self.fuse
    }
}

impl Default for FuseBuilder {
    fn default() -> Self {
        Self::new()
    }
}
