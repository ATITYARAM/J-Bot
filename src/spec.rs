use std::{
    env,
    path::PathBuf,
    process::{exit, Command},
};

fn main() {
    // Repository root
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    // Spectate project
    let spectate_dir = repo_root.join("spectate");

    if !spectate_dir.exists() {
        eprintln!("Spectate project not found:");
        eprintln!("  {}", spectate_dir.display());
        exit(1);
    }

    println!("Launching Spectate...");
    println!();

    let status = Command::new("cargo")
        .arg("run")
        .current_dir(&spectate_dir)
        .env("JBOT_ROOT", &repo_root)
        .status()
        .expect("Failed to launch Spectate");

    exit(status.code().unwrap_or(1));
}
