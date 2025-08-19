use super::config::Fuse;
use super::types::{Pattern, ScoreResult};
use crate::utils;

impl Fuse {
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
    /// # use fuse_lib::inner::config::Fuse;
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
    /// use fuse_lib::inner::config::Fuse;
    /// let fuse = Fuse::default();
    /// let pattern = fuse.create_pattern("some text");
    /// let result = fuse.search(pattern.as_ref(), "some string");
    /// ```
    ///
    /// [`create_pattern`]: #method.create_pattern
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
