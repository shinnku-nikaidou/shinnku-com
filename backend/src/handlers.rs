use anyhow::Result;
use axum::{
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::Deserialize;
use serde_json::json;

use once_cell::sync::OnceCell;
use pyo3::ffi::{self, c_str};
use pyo3::{prelude::*, types::PyDict};
use pyo3_async_runtimes::tokio::into_future;
use std::ffi::CString;
use widestring::WideCString;

static PY_RETRIEVER_INTRO: OnceCell<Py<PyAny>> = OnceCell::new();
static PY_RETRIEVER_FINDNAME: OnceCell<Py<PyAny>> = OnceCell::new();

pub fn configure_python() -> Result<()> {
    let base = std::env::current_dir()?.join(".venv");
    let exe_path = base.join("bin/python3").canonicalize()?;
    let home_path = base.canonicalize()?;

    unsafe {
        let exe = WideCString::from_str(
            exe_path
                .to_str()
                .ok_or_else(|| anyhow::anyhow!("invalid exe path"))?,
        )?;
        let home = WideCString::from_str(
            home_path
                .to_str()
                .ok_or_else(|| anyhow::anyhow!("invalid home path"))?,
        )?;
        ffi::Py_SetProgramName(exe.as_ptr() as *const libc::wchar_t);
        ffi::Py_SetPythonHome(home.as_ptr() as *const libc::wchar_t);
    }

    // Initialize the interpreter using the configured paths
    pyo3::prepare_freethreaded_python();
    Ok(())
}

pub fn init_py() -> PyResult<()> {
    Python::with_gil(|py| {
        let code = CString::new(include_str!("../../ai/preload.py"))?;
        let module =
            PyModule::from_code(py, code.as_c_str(), c_str!("preload.py"), c_str!("preload"))?;
        let intro = module.getattr("retriever_intro")?.unbind();
        let findname = module.getattr("retriever_findname")?.unbind();
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
            .bind(py);
        let kwargs = PyDict::new(py);
        kwargs.set_item("k", 1)?;
        let awaitable = retriever.call_method("ainvoke", (name,), Some(&kwargs))?;
        into_future(awaitable)
    })?;

    let obj = fut.await?;
    let content = Python::with_gil(|py| -> PyResult<Option<String>> {
        let obj = obj.bind(py);
        if obj.is_none() {
            return Ok(None);
        }
        if let Ok(v) = obj.getattr("page_content") {
            return v.extract().map(Some);
        }
        if let Ok(list) = obj.downcast::<pyo3::types::PyList>() {
            if let Some(item) = list.iter().next() {
                if let Ok(v) = item.getattr("page_content") {
                    return v.extract().map(Some);
                }
            }
        }
        Ok(Some(obj.str()?.to_str()?.to_string()))
    })?;
    Ok(content)
}

async fn fetch_findname(name: &str) -> Result<Vec<String>> {
    let fut = Python::with_gil(|py| -> PyResult<_> {
        let retriever = PY_RETRIEVER_FINDNAME
            .get()
            .expect("python not initialized")
            .bind(py);
        let kwargs = PyDict::new(py);
        kwargs.set_item("k", 12)?;
        let awaitable = retriever.call_method("ainvoke", (name,), Some(&kwargs))?;
        into_future(awaitable)
    })?;

    let obj = fut.await?;
    let list = Python::with_gil(|py| -> PyResult<Vec<String>> {
        let obj = obj.bind(py);
        if let Ok(pylist) = obj.downcast::<pyo3::types::PyList>() {
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
        } else if let Ok(v) = obj.getattr("page_content") {
            Ok(vec![v.extract::<String>()?])
        } else if obj.is_none() {
            Ok(vec![])
        } else {
            Ok(vec![obj.str()?.to_str()?.to_string()])
        }
    })?;
    Ok(list)
}
