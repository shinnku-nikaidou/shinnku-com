use crate::application::wiki::queries::get_wiki_content_query::GetWikiContentQuery;
use crate::domain::wiki::services::wiki_service::get_wiki_background;
use anyhow::Result;
use redis::aio::ConnectionManager;

/// Handler for wiki content operations
#[derive(Default)]
pub struct GetWikiContentHandler;

impl GetWikiContentHandler {
    pub fn new() -> Self {
        Self
    }

    /// Execute the get wiki content query
    ///
    /// # Errors
    ///
    /// Returns an error if the Redis operations fail
    pub async fn handle(
        &self,
        query: &GetWikiContentQuery,
        redis: &mut ConnectionManager,
    ) -> Result<Option<String>> {
        get_wiki_background(redis, &query.name).await
    }
}
