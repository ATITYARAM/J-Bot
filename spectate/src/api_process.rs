use std::sync::OnceLock;

use axum::{
    extract::{Json as JsonBody, Path},
    routing::{get, post},
    Json,
    Router,
};
use serde::{Deserialize, Serialize};

use crate::process::ProcessManager;

static PROCESS_MANAGER: OnceLock<ProcessManager> = OnceLock::new();

fn manager() -> &'static ProcessManager {
    PROCESS_MANAGER.get_or_init(ProcessManager::new)
}

#[derive(Serialize)]
pub struct ProcessInfo {
    pub id: String,
    pub title: String,
    pub command: String,
    pub interactive: bool,
    pub running: bool,
}

#[derive(Deserialize)]
pub struct InputRequest {
    pub input: String,
}

pub fn router() -> Router {
    Router::new()
        .route("/api/process", get(list))
        .route("/api/process/{id}/start", post(start))
        .route("/api/process/{id}/stop", post(stop))
        .route("/api/process/{id}/clear", post(clear))
        .route("/api/process/{id}/output", get(output))
        .route("/api/process/{id}/input", post(input))
}

async fn list() -> Json<Vec<ProcessInfo>> {

    let mgr = manager();

    let mut processes = Vec::new();

    for process in ProcessManager::definitions() {

        processes.push(ProcessInfo {

            id: process.id.to_string(),

            title: process.title.to_string(),

            command: process.command.to_string(),

            interactive: process.interactive,

            running: mgr.running(process.id),

        });

    }

    Json(processes)
}

async fn start(
    Path(id): Path<String>,
) -> Json<bool> {

    Json(manager().start(&id).is_ok())

}

async fn stop(
    Path(id): Path<String>,
) -> Json<bool> {

    Json(manager().stop(&id).is_ok())

}

async fn clear(
    Path(id): Path<String>,
) -> Json<bool> {

    manager().clear(&id);

    Json(true)

}

async fn output(
    Path(id): Path<String>,
) -> Json<Vec<String>> {

    Json(manager().output(&id))

}

async fn input(
    Path(id): Path<String>,
    JsonBody(req): JsonBody<InputRequest>,
) -> Json<bool> {

    Json(manager().send_input(&id, &req.input).is_ok())

}
