use super::config::Fuse;
use super::types::{Pattern, ScoreResult};
use crate::utils;

/// Search context containing preprocessed data for efficient searching
struct SearchContext {
    string_to_search: String,
    text_length: usize,
    location: i32,
    distance: i32,
    initial_threshold: f64,
}

/// Mutable state during the search process
struct MatchState {
    threshold: f64,
    best_location: usize,
    match_mask: Vec<u8>,
}

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

    /// Prepares the search context with preprocessed data
    fn prepare_search_context(&self, _pattern: &Pattern, string: &str) -> SearchContext {
        let string_to_search = if self.is_case_sensitive {
            string.to_owned()
        } else {
            // Only allocate when necessary
            string.to_ascii_lowercase()
        };

        let text_length = string_to_search.len();

        SearchContext {
            string_to_search,
            text_length,
            location: self.location,
            distance: self.distance,
            initial_threshold: self.threshold,
        }
    }

    /// Checks for exact match and returns result if found
    fn check_exact_match(&self, pattern: &Pattern, context: &SearchContext) -> Option<ScoreResult> {
        if pattern.text == context.string_to_search {
            Some(ScoreResult {
                score: 0.,
                ranges: vec![0..context.text_length],
            })
        } else {
            None
        }
    }

    /// Performs exact match pre-scanning and initializes match state
    fn perform_exact_prescan(&self, pattern: &Pattern, context: &SearchContext) -> MatchState {
        let string_to_search = &context.string_to_search;
        let text_length = context.text_length;
        let location = context.location;
        let distance = context.distance;
        let mut threshold = context.initial_threshold;

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

        while let Some(offset) = index {
            let i = best_location + offset;
            let score = utils::calculate_score(pattern.len, 0, i as i32, location, distance);
            threshold = threshold.min(score);
            best_location = i + pattern.len;
            index = safe_find(string_to_search, best_location, &pattern.text);

            for idx in 0..pattern.len {
                if i + idx < match_mask_arr.len() {
                    match_mask_arr[i + idx] = 1;
                }
            }
        }

        MatchState {
            threshold,
            best_location,
            match_mask: match_mask_arr,
        }
    }

    fn search_util(&self, pattern: &Pattern, string: &str) -> ScoreResult {
        let search_context = self.prepare_search_context(pattern, string);

        // Fast path: exact match
        if let Some(exact_result) = self.check_exact_match(pattern, &search_context) {
            return exact_result;
        }

        // Perform exact match pre-scanning
        let mut match_state = self.perform_exact_prescan(pattern, &search_context);

        // Perform Bitap fuzzy search
        let final_score = self.perform_bitap_search(pattern, &search_context, &mut match_state);

        ScoreResult {
            score: final_score,
            ranges: utils::find_ranges(&match_state.match_mask).unwrap_or_default(),
        }
    }

    fn perform_bitap_search(
        &self,
        pattern: &Pattern,
        context: &SearchContext,
        match_state: &mut MatchState,
    ) -> f64 {
        let string_to_search = &context.string_to_search;
        let string_chars = string_to_search.as_bytes();
        let text_length = context.text_length;
        let location = context.location;
        let distance = context.distance;
        let mut threshold = match_state.threshold;

        let mut score = 1.0;
        let mut bin_max = pattern.len + text_length;
        let mut last_bit_arr = vec![];
        let text_count = string_chars.len();

        for i in 0..pattern.len {
            let (start, finish, bin_max_new) = self.calculate_search_bounds(
                i,
                pattern,
                location,
                distance,
                text_length,
                threshold,
                bin_max,
            );
            bin_max = bin_max_new;

            if start > finish {
                continue;
            }

            let mut bit_arr = vec![0; finish + 2];
            bit_arr[finish + 1] = if i < 64 { (1 << i) - 1 } else { u64::MAX };

            let search_score = self.perform_bitap_iteration(
                pattern,
                string_to_search,
                match_state,
                &mut bit_arr,
                &last_bit_arr,
                start,
                finish,
                i,
                text_count,
                location,
                distance,
                &mut threshold,
            );

            if let Some(s) = search_score {
                score = s;
            }

            if utils::calculate_score(pattern.len, i as i32 + 1, location, location, distance)
                > threshold
            {
                break;
            }

            last_bit_arr = bit_arr.clone();
        }

        score
    }

    fn calculate_search_bounds(
        &self,
        i: usize,
        pattern: &Pattern,
        location: i32,
        distance: i32,
        text_length: usize,
        threshold: f64,
        mut bin_max: usize,
    ) -> (usize, usize, usize) {
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

        (start, finish, bin_max)
    }

    fn perform_bitap_iteration(
        &self,
        pattern: &Pattern,
        string_to_search: &str,
        match_state: &mut MatchState,
        bit_arr: &mut [u64],
        last_bit_arr: &[u64],
        start: usize,
        finish: usize,
        i: usize,
        text_count: usize,
        location: i32,
        distance: i32,
        threshold: &mut f64,
    ) -> Option<f64> {
        let mut current_location_index: usize = 0;
        let mut found_score = None;

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
                && let Some(mask_item) = match_state
                    .match_mask
                    .get_mut(current_location.saturating_sub(1))
            {
                *mask_item = 1;
            }

            bit_arr[current_location] = ((bit_arr[current_location + 1] << 1) | 1) & char_match;
            if i > 0 && !last_bit_arr.is_empty() {
                bit_arr[current_location] |=
                    (((last_bit_arr.get(current_location + 1).copied().unwrap_or(0)
                        | last_bit_arr.get(current_location).copied().unwrap_or(0))
                        << 1_u64)
                        | 1)
                        | last_bit_arr.get(current_location + 1).copied().unwrap_or(0);
            }

            if (bit_arr[current_location] & pattern.mask) != 0 {
                let score = utils::calculate_score(
                    pattern.len,
                    i as i32,
                    location,
                    current_location.saturating_sub(1) as i32,
                    distance,
                );

                if score <= *threshold {
                    *threshold = score;
                    match_state.best_location = current_location.saturating_sub(1);
                    found_score = Some(score);

                    if match_state.best_location as i32 <= location {
                        break;
                    };
                }
            }
        }

        found_score
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
