use super::config::Fuse;
use super::types::{Pattern, ScoreResult};
use crate::fuseable::Fuseable;
use crate::types::{FResult, FuseProperty, FuseableSearchResult};
use crate::utils;

/// Search context containing preprocessed data for efficient searching
struct SearchContext {
    string_to_search: String,
    text_length: usize,
    location: usize,
    distance: usize,
    initial_threshold: f64,
}

/// Mutable state during the search process
struct MatchState {
    threshold: f64,
    best_location: usize,
    match_mask: Vec<u8>,
}

impl Fuse {
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
            let score = utils::calculate_score(pattern.len, 0, i, location, distance);
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

    pub(crate) fn search_util(&self, pattern: &Pattern, string: &str) -> ScoreResult {
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

            if utils::calculate_score(pattern.len, (i + 1) as isize, location, location, distance)
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
        location: usize,
        distance: usize,
        text_length: usize,
        threshold: f64,
        mut bin_max: usize,
    ) -> (usize, usize, usize) {
        let mut bin_min = 0;
        let mut bin_mid = bin_max;

        while bin_min < bin_mid {
            if utils::calculate_score(
                pattern.len,
                i as isize,
                location,
                location + bin_mid,
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

        let start = 1_usize.max(location.saturating_sub(bin_mid).saturating_add(1));
        let finish = text_length
            .min(location.saturating_add(bin_mid))
            .saturating_add(pattern.len);

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
        location: usize,
        distance: usize,
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
                    i as isize,
                    location,
                    current_location.saturating_sub(1),
                    distance,
                );

                if score <= *threshold {
                    *threshold = score;
                    match_state.best_location = current_location.saturating_sub(1);
                    found_score = Some(score);

                    if match_state.best_location <= location {
                        break;
                    };
                }
            }
        }

        found_score
    }

    /// Searches a single fuseable item and returns the result if any fields match.
    pub(crate) fn search_fuseable_item(
        &self,
        pattern: Option<&super::types::Pattern>,
        index: usize,
        item: &impl Fuseable,
    ) -> Option<FuseableSearchResult> {
        let mut total_score = 0.0;
        let mut property_results = Vec::new();

        for property in &item.properties() {
            if let Some(field_result) = self.search_property(pattern, property, item) {
                total_score += field_result.score;
                property_results.push(field_result);
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
    }

    /// Searches a single property of a fuseable item.
    fn search_property(
        &self,
        pattern: Option<&super::types::Pattern>,
        property: &FuseProperty,
        item: &impl Fuseable,
    ) -> Option<FResult> {
        let value = item.lookup(&property.value)?;
        let search_result = self.search(pattern, value)?;

        let weight = if (property.weight - 1.0).abs() < f64::EPSILON {
            1.0
        } else {
            1.0 - property.weight
        };

        let score = if search_result.score == 0.0 && (weight - 1.0).abs() < f64::EPSILON {
            0.001
        } else {
            search_result.score * weight
        };

        Some(FResult {
            value: property.value.clone(),
            score,
            ranges: search_result.ranges,
        })
    }
}
