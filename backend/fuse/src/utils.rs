use std::collections::HashMap;
use std::ops::Range;

pub fn calculate_score(
    pattern_length: usize,
    e: usize,
    x: usize,
    loc: usize,
    distance: usize,
) -> f64 {
    let accuracy = (e as f64) / (pattern_length as f64);

    // Use abs_diff for safe absolute difference calculation
    let proximity = x.abs_diff(loc);

    if distance == 0 {
        return if proximity != 0 { 1.0 } else { accuracy };
    }
    accuracy + (proximity as f64) / (distance as f64)
}
/// Initializes the alphabet for the Bitap algorithm
/// - Parameter pattern: The text to encode.
/// - Returns: Hashmap of character locations.
pub fn calculate_pattern_alphabet(pattern: &[u8]) -> HashMap<u8, u64> {
    let len = pattern.len();
    let mut mask = HashMap::new();
    for (i, &c) in pattern.iter().enumerate() {
        mask.insert(c, mask.get(&c).unwrap_or(&0) | (1 << (len - i - 1)));
    }
    mask
}

/// Returns an array of `Range<usize>`, where each range represents a consecutive list of `1`s.
/// Uses Rust idioms with Option<usize> instead of sentinel values.
/// - Parameter mask: A byte array representing the match mask.
/// - Returns: `Vec<Range<usize>>`.
pub fn find_ranges(mask: &[u8]) -> Vec<Range<usize>> {
    if mask.is_empty() {
        return Vec::new();
    }

    let mut ranges = Vec::new();
    let mut current_start: Option<usize> = None;

    for (index, &value) in mask.iter().enumerate() {
        match (current_start, value >= 1) {
            // Starting a new range
            (None, true) => current_start = Some(index),
            // Ending a range
            (Some(start), false) => {
                ranges.push(start..index);
                current_start = None;
            }
            // Continue current state (either in range or not in range)
            _ => {}
        }
    }

    // Handle the case where the mask ends with a 1
    if let Some(start) = current_start {
        ranges.push(start..mask.len());
    }

    ranges
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_score_basic() {
        // Test basic score calculation
        let score = calculate_score(5, 1, 10, 5, 10);
        assert!(score > 0.0);
    }

    #[test]
    fn test_calculate_score_zero_distance() {
        // When distance is 0, should return accuracy if proximity is 0, otherwise 1.0
        let score_same_location = calculate_score(5, 1, 5, 5, 0);
        let expected_accuracy = 1.0 / 5.0; // e/pattern_length
        assert!((score_same_location - expected_accuracy).abs() < f64::EPSILON);

        let score_diff_location = calculate_score(5, 1, 10, 5, 0);
        assert!((score_diff_location - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_calculate_score_proximity_calculation() {
        // Test that proximity calculation works correctly for x > loc and x < loc
        let score1 = calculate_score(5, 1, 10, 5, 10); // proximity = 5
        let score2 = calculate_score(5, 1, 5, 10, 10); // proximity = 5
        assert!(
            (score1 - score2).abs() < f64::EPSILON,
            "Scores should be equal regardless of order"
        );
    }

    #[test]
    fn test_find_ranges_empty() {
        assert_eq!(find_ranges(&[]), Vec::<Range<usize>>::new());
    }

    #[test]
    fn test_find_ranges_single_range() {
        assert_eq!(find_ranges(&[1, 1, 1]), vec![0..3]);
    }

    #[test]
    fn test_find_ranges_multiple_ranges() {
        assert_eq!(find_ranges(&[1, 1, 0, 1, 0, 1, 1]), vec![0..2, 3..4, 5..7]);
    }

    #[test]
    fn test_find_ranges_no_matches() {
        assert_eq!(find_ranges(&[0, 0, 0]), Vec::<Range<usize>>::new());
    }

    #[test]
    fn test_find_ranges_ends_with_match() {
        assert_eq!(find_ranges(&[0, 1, 1]), vec![1..3]);
    }

    #[test]
    fn test_find_ranges_starts_with_match() {
        assert_eq!(find_ranges(&[1, 1, 0]), vec![0..2]);
    }
}
