use axum::{
    routing::get,
    Json,
    Router,
};

use crate::{
    models::{Project, SystemInfo, Tool},
    scan,
};

pub fn router() -> Router {
    Router::new()
        .route("/api/system", get(system))
        .route("/api/projects", get(projects))
        .route("/api/tools", get(tools))
}

async fn system() -> Json<SystemInfo> {
    Json(scan::get_system())
}

async fn projects() -> Json<Vec<Project>> {
    Json(scan::scan_projects())
}

async fn tools() -> Json<Vec<Tool>> {
    Json(scan::scan_tools())
}
