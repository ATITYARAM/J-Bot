use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SystemInfo {
    pub hostname: String,
    pub ip: String,

    pub cpu: f32,
    pub memory: String,
    pub disk: u8,
    pub network: String,
}

#[derive(Debug, Serialize)]
pub struct Project {
    pub name: String,
    pub project_type: String,
    pub branch: String,
    pub health: String,
}

#[derive(Debug, Serialize)]
pub struct Tool {
    pub group: String,
    pub name: String,
    pub source: String,
    pub target: String,
    pub status: String,
}
