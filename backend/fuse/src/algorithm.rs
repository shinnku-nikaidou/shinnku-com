use super::config::Fuse;
use super::types::{Pattern, ScoreResult};
use crate::fuseable::Fuseable;
use crate::types::{FResult, FuseProperty, FuseableSearchResult};
use crate::utils::{self, calculate_score};

/// Safe index wrapper to prevent off-by-one errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SafeIndex(usize);

impl SafeIndex {
    fn new(value: usize) -> Self {
        SafeIndex(value)
    }

    fn prev(self) -> Option<Self> {
        self.0.checked_sub(1).map(SafeIndex)
    }

    fn as_usize(self) -> usize {
        self.0
    }
}

/// Safe accessor for bit arrays with default values
struct BitArrayAccessor<'a>(&'a [u64]);

impl<'a> BitArrayAccessor<'a> {
    fn new(slice: &'a [u64]) -> Self {
        BitArrayAccessor(slice)
    }

    fn get_or_zero(&self, index: usize) -> u64 {
        self.0.get(index).copied().unwrap_or(0)
    }

    fn get_adjacent(&self, index: usize) -> (u64, u64) {
        (self.get_or_zero(index), self.get_or_zero(index + 1))
    }
}

/// Character matcher for pattern alphabet lookup
struct CharMatcher<'a> {
    text_bytes: &'a [u8],
    pattern_alphabet: &'a std::collections::HashMap<u8, u64>,
}

impl<'a> CharMatcher<'a> {
    fn new(text: &'a str, alphabet: &'a std::collections::HashMap<u8, u64>) -> Self {
        CharMatcher {
            text_bytes: text.as_bytes(),
            pattern_alphabet: alphabet,
        }
    }

    fn match_at(&self, location: SafeIndex) -> u64 {
        location
            .prev()
            .and_then(|idx| self.text_bytes.get(idx.as_usize()))
            .and_then(|&byte| self.pattern_alphabet.get(&byte))
            .copied()
            .unwrap_or(0)
    }
}

/// Represents search bounds with safe iteration
#[derive(Debug)]
struct SearchBounds {
    start: usize,
    finish: usize,
}

impl SearchBounds {
    fn calculate(location: usize, bin_mid: usize, text_length: usize, pattern_len: usize) -> Self {
        let start = 1.max(location.saturating_sub(bin_mid).saturating_add(1));
        let finish = text_length
            .min(location.saturating_add(bin_mid))
            .saturating_add(pattern_len);

        Self { start, finish }
    }

    fn is_valid(&self) -> bool {
        self.start <= self.finish
    }

    fn iter_reverse(&self) -> impl Iterator<Item = SafeIndex> {
        (self.start..=self.finish).rev().map(SafeIndex::new)
    }
}

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
                #[allow(clippy::single_range_in_vec_init)]
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
            let score = calculate_score(pattern.len, 0, i, location, distance);
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
            ranges: utils::find_ranges(&match_state.match_mask),
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
            let (bounds, bin_max_new) = self.calculate_search_bounds(
                i,
                pattern,
                location,
                distance,
                text_length,
                threshold,
                bin_max,
            );
            bin_max = bin_max_new;

            if !bounds.is_valid() {
                continue;
            }

            let mut bit_arr = vec![0; bounds.finish + 2];
            bit_arr[bounds.finish + 1] = if i < 64 { (1 << i) - 1 } else { u64::MAX };

            let search_score = self.perform_bitap_iteration(
                pattern,
                string_to_search,
                match_state,
                &mut bit_arr,
                &last_bit_arr,
                bounds.start,
                bounds.finish,
                i,
                text_count,
                location,
                distance,
                &mut threshold,
            );

            if let Some(s) = search_score {
                score = s;
            }

            if calculate_score(pattern.len, i + 1, location, location, distance) > threshold {
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
    ) -> (SearchBounds, usize) {
        let mut bin_min = 0;
        let mut bin_mid = bin_max;

        while bin_min < bin_mid {
            if calculate_score(pattern.len, i, location, location + bin_mid, distance) <= threshold
            {
                bin_min = bin_mid;
            } else {
                bin_max = bin_mid;
            }
            bin_mid = ((bin_max - bin_min) / 2) + bin_min;
        }
        bin_max = bin_mid;

        let bounds = SearchBounds::calculate(location, bin_mid, text_length, pattern.len);
        (bounds, bin_max)
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
        let char_matcher = CharMatcher::new(string_to_search, &pattern.alphabet);
        let last_bit_accessor = BitArrayAccessor::new(last_bit_arr);
        let bounds = SearchBounds { start, finish };
        let mut found_score = None;

        for current_location in bounds.iter_reverse() {
            let char_match = if current_location.as_usize() <= text_count {
                char_matcher.match_at(current_location)
            } else {
                0
            };

            // Update match mask if character matches
            if char_match != 0
                && let Some(mask_item) = current_location
                    .prev()
                    .and_then(|idx| match_state.match_mask.get_mut(idx.as_usize()))
            {
                *mask_item = 1;
            }

            let current_idx = current_location.as_usize();

            // Calculate bit array value
            bit_arr[current_idx] = ((bit_arr[current_idx + 1] << 1) | 1) & char_match;

            if i > 0 && !last_bit_arr.is_empty() {
                let (last_current, last_next) = last_bit_accessor.get_adjacent(current_idx);
                bit_arr[current_idx] |= (((last_next | last_current) << 1) | 1) | last_next;
            }

            if (bit_arr[current_idx] & pattern.mask) != 0 {
                let score_location = current_location
                    .prev()
                    .map(|idx| idx.as_usize())
                    .unwrap_or(0);

                let score = calculate_score(pattern.len, i, location, score_location, distance);

                if score <= *threshold {
                    *threshold = score;
                    match_state.best_location = score_location;
                    found_score = Some(score);

                    if match_state.best_location <= location {
                        break;
                    }
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
