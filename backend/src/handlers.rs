use anyhow::Result;
use axum::{
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::Deserialize;
use serde_json::json;

use once_cell::sync::OnceCell;
use pyo3::{prelude::*, types::PyDict};
use pyo3_asyncio::tokio::into_future;

static PY_RETRIEVER_INTRO: OnceCell<Py<PyAny>> = OnceCell::new();
static PY_RETRIEVER_FINDNAME: OnceCell<Py<PyAny>> = OnceCell::new();

pub fn init_py() -> PyResult<()> {
    Python::with_gil(|py| {
        let module = PyModule::from_code(
            py,
            include_str!("../../ai/preload.py"),
            "preload.py",
            "preload",
        )?;
        let intro = module.getattr("retriever_intro")?.into_py(py);
        let findname = module.getattr("retriever_findname")?.into_py(py);
        PY_RETRIEVER_INTRO.set(intro).ok();
        PY_RETRIEVER_FINDNAME.set(findname).ok();
        Ok(())
    })
}

#[derive(Deserialize)]
pub struct NameQuery {
    pub name: Option<String>,
}

pub async fn intro(Query(params): Query<NameQuery>) -> impl IntoResponse {
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

pub async fn find_name(Query(params): Query<NameQuery>) -> impl IntoResponse {
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

async fn fetch_intro(name: &str) -> Result<Option<String>> {
    let fut = Python::with_gil(|py| -> PyResult<_> {
        let retriever = PY_RETRIEVER_INTRO
            .get()
            .expect("python not initialized")
            .as_ref(py);
        let kwargs = PyDict::new(py);
        kwargs.set_item("k", 1)?;
        let awaitable = retriever.call_method("ainvoke", (name,), Some(kwargs))?;
        into_future(awaitable)
    })?;

    let obj = fut.await?;
    let content = Python::with_gil(|py| -> PyResult<Option<String>> {
        if obj.is_none(py) {
            return Ok(None);
        }
        if let Ok(v) = obj.as_ref(py).getattr("page_content") {
            return v.extract().map(Some);
        }
        if let Ok(list) = obj.as_ref(py).downcast::<pyo3::types::PyList>() {
            if let Some(item) = list.iter().next() {
                if let Ok(v) = item.getattr("page_content") {
                    return v.extract().map(Some);
                }
            }
        }
        Ok(Some(obj.as_ref(py).str()?.to_str()?.to_string()))
    })?;
    Ok(content)
}

async fn fetch_findname(name: &str) -> Result<Vec<String>> {
    let fut = Python::with_gil(|py| -> PyResult<_> {
        let retriever = PY_RETRIEVER_FINDNAME
            .get()
            .expect("python not initialized")
            .as_ref(py);
        let kwargs = PyDict::new(py);
        kwargs.set_item("k", 12)?;
        let awaitable = retriever.call_method("ainvoke", (name,), Some(kwargs))?;
        into_future(awaitable)
    })?;

    let obj = fut.await?;
    let list = Python::with_gil(|py| -> PyResult<Vec<String>> {
        if let Ok(pylist) = obj.as_ref(py).downcast::<pyo3::types::PyList>() {
            Ok(pylist
                .iter()
                .filter_map(|item| {
                    if let Ok(v) = item.getattr("page_content") {
                        v.extract::<String>().ok()
                    } else {
                        item.extract::<String>().ok()
                    }
                })
                .collect())
        } else if let Ok(v) = obj.as_ref(py).getattr("page_content") {
            Ok(vec![v.extract::<String>()?])
        } else if obj.is_none(py) {
            Ok(vec![])
        } else {
            Ok(vec![obj.as_ref(py).str()?.to_str()?.to_string()])
        }
    })?;
    Ok(list)
}
