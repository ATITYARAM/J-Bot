use std::sync::OnceLock;

use axum::{
    extract::Path,
    routing::{get, post},
    Json,
    Router,
};
use serde::Serialize;

use crate::process::ProcessManager;

static PROCESS_MANAGER: OnceLock<ProcessManager> = OnceLock::new();

fn manager() -> &'static ProcessManager {
    PROCESS_MANAGER.get_or_init(ProcessManager::new)
}

#[derive(Serialize)]
struct ProcessInfo {
    name: String,
    running: bool,
}

pub fn router() -> Router {
    Router::new()
        .route("/api/process", get(list_processes))
        .route("/api/process/{name}/start", post(start_process))
        .route("/api/process/{name}/stop", post(stop_process))
        .route("/api/process/{name}/clear", post(clear_output))
        .route("/api/process/{name}/output", get(process_output))
}

async fn list_processes() -> Json<Vec<ProcessInfo>> {
    let mgr = manager();

    Json(vec![
        ProcessInfo {
            name: "teleop".to_string(),
            running: mgr.running("teleop"),
        },
        ProcessInfo {
            name: "build".to_string(),
            running: mgr.running("build"),
        },
        ProcessInfo {
            name: "s3".to_string(),
            running: mgr.running("s3"),
        },
    ])
}

async fn start_process(
    Path(name): Path<String>,
) -> Json<bool> {
    Json(manager().start(&name).is_ok())
}

async fn stop_process(
    Path(name): Path<String>,
) -> Json<bool> {
    Json(manager().stop(&name).is_ok())
}

async fn clear_output(
    Path(name): Path<String>,
) -> Json<bool> {
    manager().clear(&name);

    Json(true)
}

async fn process_output(
    Path(name): Path<String>,
) -> Json<Vec<String>> {
    Json(manager().output(&name))
}
