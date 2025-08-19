use crate::domain::search::entities::search_item::{SearchItem, SearchList};
use crate::domain::search::repositories::fuzzy_search_repository::FuzzySearchRepository;
use fuse_lib::config::Fuse;
use fuse_lib::fuseable::Fuseable;
use fuse_lib::types::FuseProperty;
use std::collections::HashMap;

/// Configuration for the Fuse search engine
#[derive(Debug, Clone)]
pub struct FuseConfig {
    pub threshold: f64,
    pub distance: usize,
    pub max_pattern_length: usize,
    pub is_case_sensitive: bool,
    pub tokenize: bool,
}

impl Default for FuseConfig {
    fn default() -> Self {
        Self {
            threshold: 0.6,
            distance: 800,
            max_pattern_length: 32,
            is_case_sensitive: false,
            tokenize: true,
        }
    }
}

/// Adapter that implements fuzzy search using the Fuse library
///
/// This adapter wraps the fuse-lib dependency and implements the domain's
/// FuzzySearchRepository interface, keeping the domain layer free from
/// external dependencies.
pub struct FuseSearchAdapter {
    config: FuseConfig,
}

impl FuseSearchAdapter {
    pub fn new(config: FuseConfig) -> Self {
        Self { config }
    }

    pub fn with_default_config() -> Self {
        Self::new(FuseConfig::default())
    }

    /// Create a Fuse instance with the current configuration
    fn create_fuse(&self) -> Fuse {
        Fuse {
            threshold: self.config.threshold,
            distance: self.config.distance,
            max_pattern_length: self.config.max_pattern_length,
            is_case_sensitive: self.config.is_case_sensitive,
            tokenize: self.config.tokenize,
            ..Default::default()
        }
    }
}

/// Wrapper to make SearchItem compatible with Fuse library
///
/// This wrapper implements the Fuseable trait required by the fuse library,
/// keeping this external dependency concern isolated in the infrastructure layer.
struct FuseableSearchItem<'a>(&'a SearchItem);

impl<'a> Fuseable for FuseableSearchItem<'a> {
    fn properties(&self) -> Vec<FuseProperty> {
        vec![FuseProperty::init("id")]
    }

    fn lookup(&self, key: &str) -> Option<&str> {
        if key == "id" { Some(&self.0.id) } else { None }
    }
}

impl FuzzySearchRepository for FuseSearchAdapter {
    fn search(&self, query: &str, items: &SearchList) -> SearchList {
        let fuse = self.create_fuse();

        // Convert SearchItems to FuseableSearchItems for the fuse library
        let fuseable_items: Vec<FuseableSearchItem> =
            items.iter().map(FuseableSearchItem).collect();

        let results = fuse.search_text_in_fuse_list(query, &fuseable_items);

        // Convert results back to SearchItems
        results
            .into_iter()
            .map(|r| items[r.index].clone())
            .collect()
    }

    fn combined_search(&self, q1: &str, q2: &str, limit: usize, items: &SearchList) -> SearchList {
        let fuse = self.create_fuse();

        // Convert SearchItems to FuseableSearchItems
        let fuseable_items: Vec<FuseableSearchItem> =
            items.iter().map(FuseableSearchItem).collect();

        let q1_results = fuse.search_text_in_fuse_list(q1, &fuseable_items);
        let q2_results = fuse.search_text_in_fuse_list(q2, &fuseable_items);

        let mut scores: HashMap<usize, f64> = HashMap::new();

        // Process first query results
        for result in q1_results {
            scores.insert(result.index, result.score);
        }

        // Process second query results and combine scores
        for result in q2_results {
            scores
                .entry(result.index)
                .and_modify(|s| *s = (*s + result.score) / 2.0)
                .or_insert(result.score);
        }

        // Sort by score and take top results
        let mut scored_items: Vec<(SearchItem, f64)> = scores
            .into_iter()
            .map(|(idx, score)| (items[idx].clone(), score))
            .collect();

        scored_items.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

        scored_items
            .into_iter()
            .take(limit)
            .map(|(item, _)| item)
            .collect()
    }
}
