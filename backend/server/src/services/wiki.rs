use anyhow::Result;
use redis::aio::ConnectionManager;

/// Fetch the background image URL for a wiki page by name.
/// Returns `Ok(None)` if no cached entry is found.
pub async fn get_wiki_background(
    con: &mut ConnectionManager,
    name: &str,
) -> Result<Option<String>> {
    let key_search = format!("cache:search:wiki:zh:{name}");
    let pageid: Option<String> = match redis::cmd("GET").arg(&key_search).query_async(con).await {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Redis GET {key_search} error: {e}");
            return Ok(None);
        }
    };

    if let Some(pageid) = pageid {
        let key_img = format!("img:wiki:zh:{pageid}");
        match redis::cmd("GET").arg(&key_img).query_async(con).await {
            Ok(bg) => Ok(bg),
            Err(e) => {
                eprintln!("Redis GET {key_img} error: {e}");
                Ok(None)
            }
        }
    } else {
        Ok(None)
    }
}
