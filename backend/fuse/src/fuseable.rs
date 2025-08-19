use super::config::Fuse;
use super::types::{FResult, FuseProperty, FuseableSearchResult};

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
