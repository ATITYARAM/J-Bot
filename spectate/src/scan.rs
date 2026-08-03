use std::{
    env,
    fs,
    path::Path,
};

use crate::models::{Project, SystemInfo, Tool};

pub fn get_system() -> SystemInfo {
    SystemInfo {
        hostname: hostname(),
        ip: local_ip(),

        cpu: 15.0,
        memory: "4.2 / 16 GB".to_string(),
        disk: 42,
        network: "Connected".to_string(),
    }
}

pub fn scan_projects() -> Vec<Project> {
    vec![
        Project {
            name: "Spectate".to_string(),
            project_type: "Rust".to_string(),
            branch: "main".to_string(),
            health: "Healthy".to_string(),
        },
        Project {
            name: "Robot WS".to_string(),
            project_type: "ROS2".to_string(),
            branch: "jazzy".to_string(),
            health: "Healthy".to_string(),
        },
    ]
}

pub fn scan_tools() -> Vec<Tool> {
    let mut tools = Vec::new();

    let root = env::var("JBOT_ROOT").unwrap_or_else(|_| ".".to_string());
    let tools_dir = Path::new(&root).join("tools");

    scan_directory(&tools_dir, &tools_dir, &mut tools);

    tools
}

fn scan_directory(root: &Path, dir: &Path, tools: &mut Vec<Tool>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();

        if let Ok(target) = fs::read_link(&path) {
            let parent = path.parent().unwrap_or(root);

            let group = parent
                .strip_prefix(root)
                .unwrap_or(Path::new(""))
                .to_string_lossy()
                .replace('\\', "/");

            let status = if target.exists() {
                "Healthy"
            } else {
                "Broken"
            };

            tools.push(Tool {
                group,
                name: entry.file_name().to_string_lossy().into_owned(),
                source: path.display().to_string(),
                target: target.display().to_string(),
                status: status.to_string(),
            });

            continue;
        }

        if path.is_dir() {
            scan_directory(root, &path, tools);
        }
    }
}

fn hostname() -> String {
    hostname::get()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned()
}

fn local_ip() -> String {
    local_ip_address::local_ip()
        .map(|ip| ip.to_string())
        .unwrap_or_else(|_| "127.0.0.1".to_string())
}
