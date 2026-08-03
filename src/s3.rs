use std::{
    env,
    path::PathBuf,
    process::{Command, exit},
};

fn main() {
    // Repository root
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    // s3-pico crate
    let s3_pico = repo_root.join("s3-pico");

    let status = Command::new("cargo")
        .arg("run")
        .current_dir(&s3_pico)
        .env("JBOT_ROOT", &repo_root)
        .status()
        .expect("failed to start s3-pico");

    exit(status.code().unwrap_or(1));
}
