use serde::Deserialize;

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: Option<String>,
    pub n: Option<usize>,
}

#[derive(Deserialize)]
pub struct CombineSearchQuery {
    pub q1: Option<String>,
    pub q2: Option<String>,
    pub n: Option<usize>,
}
