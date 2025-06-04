use axum::{
    Router,
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::get,
};
use serde::Deserialize;
use serde_json::json;

/// Simple in-memory data used by the example retrievers
static INTRO_DB: &[(&str, &str)] = &[
    ("kant", "German philosopher known for his moral philosophy."),
    ("tesla", "Serbian-American inventor and engineer."),
    ("einstein", "Physicist who developed the theory of relativity."),
];

static FINDNAME_DB: &[(&str, &str)] = &[
    (
        "Immanuel Kant",
        "Prominent figure of the Enlightenment and author of the 'Critique of Pure Reason'.",
    ),
    (
        "Nikola Tesla",
        "Inventor who contributed to the design of the modern alternating current electricity supply system.",
    ),
    (
        "Albert Einstein",
        "Theoretical physicist famous for the theory of relativity and mass–energy equivalence formula E=mc^2.",
    ),
];

#[derive(Deserialize)]
struct NameQuery {
    name: Option<String>,
}

async fn intro(Query(params): Query<NameQuery>) -> impl IntoResponse {
    let ramdonshit = "Two things awe me most, the starry sky above me and the moral law within me.\n    ~ Immanuel Kant\n\n";
    if let Some(name) = params.name {
        match fetch_intro(&name).await {
            Ok(Some(content)) => (StatusCode::OK, content).into_response(),
            Ok(None) => (StatusCode::OK, "No results found.".to_string()).into_response(),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("An error occurred: {}", e),
            )
                .into_response(),
        }
    } else {
        (StatusCode::BAD_REQUEST, ramdonshit.to_string()).into_response()
    }
}

async fn find_name(Query(params): Query<NameQuery>) -> impl IntoResponse {
    let ramdonshit = "Two things awe me most, the starry sky above me and the moral law within me.\n    ~ Immanuel Kant\n\n";
    if let Some(name) = params.name {
        match fetch_findname(&name).await {
            Ok(results) => {
                if results.is_empty() {
                    (StatusCode::NOT_FOUND, Json(json!({"ans": []}))).into_response()
                } else {
                    (StatusCode::OK, Json(json!({"ans": results}))).into_response()
                }
            }
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"ans": [], "error": format!("An error occurred: {}", e)})),
            )
                .into_response(),
        }
    } else {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({"message": ramdonshit})),
        )
            .into_response()
    }
}

async fn fetch_intro(
    name: &str,
) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> {
    Ok(INTRO_DB
        .iter()
        .find(|(n, _)| n.eq_ignore_ascii_case(name))
        .map(|(_, intro)| intro.to_string()))
}

async fn fetch_findname(
    name: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
    let name_lower = name.to_lowercase();
    Ok(
        FINDNAME_DB
            .iter()
            .filter(|(n, _)| n.to_lowercase().contains(&name_lower))
            .map(|(_, prompt)| prompt.to_string())
            .collect(),
    )
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/intro", get(intro))
        .route("/findname", get(find_name));

    let listener = tokio::net::TcpListener::bind(("127.0.0.1", 2998))
        .await
        .unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
