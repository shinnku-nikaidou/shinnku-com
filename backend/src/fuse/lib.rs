use super::utils;

#[cfg(any(feature = "async", feature = "rayon"))]
use std::sync::{Arc, Mutex};

/// Required for scoped threads
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
    text: String,
    len: usize,
    mask: u64,
    alphabet: HashMap<u8, u64>,
}

/// Return type for performing a search on a list of strings
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
    /// Creates a pattern object from input string.
    ///
    /// - Parameter string: A string from which to create the pattern object
    /// - Returns: A tuple containing pattern metadata
    pub fn create_pattern(&self, string: &str) -> Option<Pattern> {
        let lowercase = string.to_lowercase();
        let pattern = if self.is_case_sensitive {
            string
        } else {
            &lowercase
        };
        let pattern_chars = pattern.as_bytes();
        let len = pattern_chars.len();

        if len == 0 {
            None
        } else {
            let alphabet = utils::calculate_pattern_alphabet(pattern_chars);
            let new_pattern = Pattern {
                text: String::from(pattern),
                len,
                mask: 1 << (len - 1),
                alphabet,
            };
            Some(new_pattern)
        }
    }

    #[allow(clippy::single_range_in_vec_init)]
    fn search_util(&self, pattern: &Pattern, string: &str) -> ScoreResult {
        let string = if self.is_case_sensitive {
            String::from(string)
        } else {
            string.to_ascii_lowercase()
        };

        let string_chars = string.as_bytes();
        let text_length = string.len();

        // Exact match
        if pattern.text == string {
            return ScoreResult {
                score: 0.,
                ranges: vec![0..text_length],
            };
        }

        let location = self.location;
        let distance = self.distance;
        let mut threshold = self.threshold;

        let mut best_location = string.find(&pattern.text).unwrap_or(0_usize);

        let mut match_mask_arr = vec![0; text_length];

        let mut index = string[best_location..].find(&pattern.text);

        let mut score;

        while index.is_some() {
            let i = best_location + index.unwrap();
            score = utils::calculate_score(pattern.len, 0, i as i32, location, distance);

            threshold = threshold.min(score);

            best_location = i + pattern.len;

            index = string[best_location..].find(&pattern.text);

            for idx in 0..pattern.len {
                match_mask_arr[i + idx] = 1;
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

            let start = 1.max(location - bin_mid as i32 + 1) as usize;
            let finish = text_length.min(location as usize + bin_mid) + pattern.len;

            let mut bit_arr = vec![0; finish + 2];

            bit_arr[finish + 1] = (1 << i) - 1;

            if start > finish {
                continue;
            };

            let mut current_location_index: usize = 0;
            for j in (start as u64..=finish as u64).rev() {
                let current_location: usize = (j - 1) as usize;
                let char_match: u64 = *(if current_location < text_count {
                    current_location_index = current_location_index
                        .checked_sub(1)
                        .unwrap_or(current_location);
                    pattern
                        .alphabet
                        .get(string.as_bytes().get(current_location_index).unwrap())
                } else {
                    None
                })
                .unwrap_or(&0);

                if char_match != 0 {
                    match_mask_arr[current_location] = 1;
                }

                let j2 = j as usize;
                bit_arr[j2] = ((bit_arr[j2 + 1] << 1) | 1) & char_match;
                if i > 0 {
                    bit_arr[j2] |= (((last_bit_arr[j2 + 1] | last_bit_arr[j2]) << 1_u64) | 1)
                        | last_bit_arr[j2 + 1];
                };

                if (bit_arr[j2] & pattern.mask) != 0 {
                    score = utils::calculate_score(
                        pattern.len,
                        i as i32,
                        location,
                        current_location as i32,
                        distance,
                    );

                    if score <= threshold {
                        threshold = score;
                        best_location = current_location;

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
            ranges: utils::find_ranges(&match_mask_arr).unwrap(),
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
                .filter_map(|x| self.create_pattern(x));

            let full_pattern_result = self.search_util(pattern, string);

            let (length, results) = word_patterns.fold(
                (0, full_pattern_result),
                |(n, mut total_result), pattern| {
                    let mut result = self.search_util(&pattern, string);
                    total_result.score += result.score;
                    total_result.ranges.append(&mut result.ranges);
                    (n + 1, total_result)
                },
            );

            let averaged_result = ScoreResult {
                score: results.score / (length + 1) as f64,
                ranges: results.ranges,
            };

            if (averaged_result.score - 1.0).abs() < 0.00001 {
                None
            } else {
                Some(averaged_result)
            }
        } else {
            let result = self.search_util(pattern, string);
            if (result.score - 1.0).abs() < 0.00001 {
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
    /// Searches for a text pattern in a given string.
    /// - Parameters:
    ///   - text: the text string to search for.
    ///   - string: The string in which to search for the pattern
    /// - Returns: Some(ScoreResult) if a match is found, containing a `score` between `0.0` (exact match) and `1` (not a match), and `ranges` of the matched characters. Otherwise if a match is not found, returns None.
    /// # Examples:
    /// ```no_run
    /// use crate::fuse::lib::{ Fuse };
    /// let fuse = Fuse::default();
    /// fuse.search_text_in_string("some text", "some string");
    /// ```
    /// **Note**: if the same text needs to be searched across many strings, consider creating the pattern once via `createPattern`, and then use the other `search` function. This will improve performance, as the pattern object would only be created once, and re-used across every search call:
    /// ```no_run
    /// use crate::fuse::lib::{ Fuse };
    /// let fuse = Fuse::default();
    /// let pattern = fuse.create_pattern("some text");
    /// fuse.search(pattern.as_ref(), "some string");
    /// fuse.search(pattern.as_ref(), "another string");
    /// fuse.search(pattern.as_ref(), "yet another string");
    /// ```
    pub fn search_text_in_string(&self, text: &str, string: &str) -> Option<ScoreResult> {
        self.search(self.create_pattern(text).as_ref(), string)
    }

    /// Searches for a text pattern in an iterable containing string references.
    ///
    /// - Parameters:
    ///   - text: The pattern string to search for
    ///   - list: Iterable over string references
    /// - Returns: Vec<SearchResult> containing Search results corresponding to matches found, with its `index`, its `score`, and the `ranges` of the matched characters.
    ///
    /// # Example:
    /// ```no_run
    /// use crate::fuse::lib::{ Fuse };
    /// let fuse = Fuse::default();
    /// let books = [
    ///     "The Silmarillion",
    ///     "The Lock Artist",
    ///     "The Lost Symbol"
    /// ];
    ///
    /// let results = fuse.search_text_in_iterable("Te silm", books.iter());
    /// ```
    pub fn search_text_in_iterable<It>(&self, text: &str, list: It) -> Vec<SearchResult>
    where
        It: IntoIterator,
        It::Item: AsRef<str>,
    {
        let pattern = self.create_pattern(text);
        let mut items = vec![];

        for (index, item) in list.into_iter().enumerate() {
            if let Some(result) = self.search(pattern.as_ref(), item.as_ref()) {
                items.push(SearchResult {
                    index,
                    score: result.score,
                    ranges: result.ranges,
                })
            }
        }
        items.sort_unstable_by(|a, b| a.score.partial_cmp(&b.score).unwrap());
        items
    }

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
        let mut result = vec![];
        for (index, item) in list.iter().enumerate() {
            let mut scores = vec![];
            let mut total_score = 0.0;

            let mut property_results = vec![];
            item.properties().iter().for_each(|property| {
                let value = item.lookup(&property.value).unwrap_or_else(|| {
                    panic!(
                        "Lookup Failed: Lookup doesnt contain requested value => {}.",
                        &property.value
                    );
                });
                if let Some(result) = self.search(pattern.as_ref(), value) {
                    let weight = if (property.weight - 1.0).abs() < 0.00001 {
                        1.0
                    } else {
                        1.0 - property.weight
                    };
                    let score = if result.score == 0.0 && (weight - 1.0).abs() < f64::EPSILON {
                        0.001
                    } else {
                        result.score
                    } * weight;
                    total_score += score;

                    scores.push(score);

                    property_results.push(FResult {
                        value: String::from(&property.value),
                        score,
                        ranges: result.ranges,
                    });
                }
            });
            if scores.is_empty() {
                continue;
            }

            let count = scores.len() as f64;
            result.push(FuseableSearchResult {
                index,
                score: total_score / count,
                results: property_results,
            })
        }

        result.sort_unstable_by(|a, b| a.score.partial_cmp(&b.score).unwrap());
        result
    }
}

#[cfg(feature = "rayon")]
impl Fuse {
    /// Asynchronously searches for a text pattern in a slice of string references.
    ///
    /// - Parameters:
    ///   - text: The pattern string to search for
    ///   - list: &[&str] A reference to a slice of string references.
    ///   - chunkSize: The size of a single chunk of the array. For example, if the slice has `1000` items, it may be useful to split the work into 10 chunks of 100. This should ideally speed up the search logic.
    ///   - completion: The handler which is executed upon completion
    ///
    /// # Example:
    /// ```no_run
    /// use crate::fuse::lib::{ Fuse, SearchResult };
    /// let fuse = Fuse::default();
    /// let books = [
    ///     "The Silmarillion",
    ///     "The Lock Artist",
    ///     "The Lost Symbol"
    /// ];
    ///
    /// fuse.search_text_in_string_list_rayon("Te silm", &books, 100 as usize, &|x: Vec<SearchResult>| {
    ///     dbg!(x);
    /// });
    /// ```
    pub fn search_text_in_string_list_rayon(
        &self,
        text: &str,
        list: &[&str],
        chunk_size: usize,
        completion: &dyn Fn(Vec<SearchResult>),
    ) {
        let pattern = Arc::new(self.create_pattern(text));

        let item_queue = Arc::new(Mutex::new(Some(vec![])));
        let count = list.len();

        rayon::scope(|scope| {
            (0..=count).step_by(chunk_size).for_each(|offset| {
                let chunk = &list[offset..count.min(offset + chunk_size)];
                let queue_ref = Arc::clone(&item_queue);
                let pattern_ref = Arc::clone(&pattern);
                scope.spawn(move |_| {
                    let mut chunk_items = vec![];

                    for (index, item) in chunk.iter().enumerate() {
                        if let Some(result) = self.search((*pattern_ref).as_ref(), item) {
                            chunk_items.push(SearchResult {
                                index: offset + index,
                                score: result.score,
                                ranges: result.ranges,
                            });
                        }
                    }

                    let mut inner_ref = queue_ref.lock().unwrap();
                    if let Some(item_queue) = inner_ref.as_mut() {
                        item_queue.append(&mut chunk_items);
                    }
                });
            });
        });

        let mut items = Arc::try_unwrap(item_queue)
            .ok()
            .unwrap()
            .into_inner()
            .unwrap()
            .unwrap();
        items.sort_unstable_by(|a, b| a.score.partial_cmp(&b.score).unwrap());
        completion(items);
    }

    /// Asynchronously searches for a text pattern in an array of `Fuseable` objects.
    /// - Parameters:
    ///   - text: The pattern string to search for
    ///   - list: A list of `Fuseable` objects, i.e. structs implementing the Fuseable trait in which to search
    ///   - chunkSize: The size of a single chunk of the array. For example, if the array has `1000` items, it may be useful to split the work into 10 chunks of 100. This should ideally speed up the search logic. Defaults to `100`.
    ///   - completion: The handler which is executed upon completion
    ///     Each `Fuseable` object contains a `properties` method which returns `FuseProperty` array. Each `FuseProperty` is a struct containing a `value` (the name of the field which should be included in the search), and a `weight` (how much "weight" to assign to the score)
    ///
    /// # Example
    /// ```no_run
    /// # use crate::fuse::lib::{ Fuse, Fuseable, FuseProperty, FuseableSearchResult };
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
    /// let results = fuse.search_text_in_fuse_list_with_chunk_size_rayon("man", &books, 1, &|x: Vec<FuseableSearchResult>| {
    ///     dbg!(x);
    /// });
    /// ```
    pub fn search_text_in_fuse_list_with_chunk_size_rayon<T>(
        &self,
        text: &str,
        list: &[T],
        chunk_size: usize,
        completion: &dyn Fn(Vec<FuseableSearchResult>),
    ) where
        T: Fuseable + std::marker::Sync,
    {
        let pattern = Arc::new(self.create_pattern(text));

        let item_queue = Arc::new(Mutex::new(Some(vec![])));
        let count = list.len();

        rayon::scope(|scope| {
            (0..=count).step_by(chunk_size).for_each(|offset| {
                let chunk = &list[offset..count.min(offset + chunk_size)];
                let queue_ref = Arc::clone(&item_queue);
                let pattern_ref = Arc::clone(&pattern);
                scope.spawn(move |_| {
                    let mut chunk_items = vec![];

                    for (index, item) in chunk.iter().enumerate() {
                        let mut scores = vec![];
                        let mut total_score = 0.0;

                        let mut property_results = vec![];
                        item.properties().iter().for_each(|property| {
                            let value = item.lookup(&property.value).unwrap_or_else(|| {
                                panic!(
                                    "Lookup doesnt contain requested value => {}.",
                                    &property.value
                                )
                            });
                            if let Some(result) = self.search((*pattern_ref).as_ref(), value) {
                                let weight = if (property.weight - 1.0).abs() < 0.00001 {
                                    1.0
                                } else {
                                    1.0 - property.weight
                                };
                                // let score = if result.score == 0.0 && weight == 1.0 { 0.001 } else { result.score } * weight;
                                let score = result.score * weight;
                                total_score += score;

                                scores.push(score);

                                property_results.push(FResult {
                                    value: String::from(&property.value),
                                    score,
                                    ranges: result.ranges,
                                });
                            }
                        });

                        if scores.is_empty() {
                            continue;
                        }

                        let count = scores.len() as f64;
                        chunk_items.push(FuseableSearchResult {
                            index,
                            score: total_score / count,
                            results: property_results,
                        })
                    }

                    let mut inner_ref = queue_ref.lock().unwrap();
                    if let Some(item_queue) = inner_ref.as_mut() {
                        item_queue.append(&mut chunk_items);
                    }
                });
            });
        });

        let mut items = Arc::try_unwrap(item_queue)
            .ok()
            .unwrap()
            .into_inner()
            .unwrap()
            .unwrap();
        items.sort_unstable_by(|a, b| a.score.partial_cmp(&b.score).unwrap());
        completion(items);
    }
}
