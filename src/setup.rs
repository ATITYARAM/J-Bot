use std::{
    fs,
    path::Path,
    process::Command,
};

pub fn run() {
    println!("=========================");
    println!("J-BOT Environment");
    println!("=========================\n");

    create_directories();

    check_os();
    check_rust();
    check_path();
    check_esp();
    check_xtensa();
    setup_environment();
    create_symlinks();
    check_tools();

    println!();

    verify_board();
}

fn create_directories() {
    use std::fs;

    let dirs = [
        "tools",
        "tools/bin",
    ];

    for dir in dirs {
        match fs::create_dir_all(dir) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Failed to create {} : {}", dir, e);
            }
        }
    }
}

fn check_os() {
    let os = fs::read_to_string("/etc/os-release").unwrap_or_default();

    if os.contains("Ubuntu") {
        println!("✓ Ubuntu");
    } else {
        println!("✗ Unsupported OS");
    }
}

fn check_rust() {
    check("cargo");
    version("cargo", "--version");
    version("rustc", "--version");
    version("rustup", "--version");

    check_path();
}

fn check_esp() {
    ensure_tool(
        "espup",
        &["cargo", "install", "espup"],
    );

    ensure_tool(
        "espflash",
        &["cargo", "install", "espflash"],
    );

    ensure_tool(
        "cargo-generate",
        &["cargo", "install", "cargo-generate"],
    );

    ensure_tool(
        "ldproxy",
        &["cargo", "install", "ldproxy"],
    );

    ensure_tool(
        "esp-generate",
        &[
            "cargo",
            "install",
            "esp-generate",
        ],
    );
}

fn check_xtensa() {
    let home = std::env::var("HOME").unwrap();

    let root = format!("{home}/.rustup/toolchains/esp");

    if find_xtensa(&root) {
        println!("✓ xtensa-esp32s3-elf-gcc");
        return;
    }

    println!("Installing Xtensa Toolchain...");

    let _ = Command::new("espup")
        .arg("install")
        .status();

    if find_xtensa(&root) {
        println!("✓ xtensa-esp32s3-elf-gcc");
    } else {
        println!("✗ xtensa-esp32s3-elf-gcc");
    }
}

fn find_xtensa(dir: &str) -> bool {
    fn walk(path: &Path) -> bool {
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                let path = entry.path();

                if path.is_dir() {
                    if walk(&path) {
                        return true;
                    }
                } else if let Some(name) = path.file_name() {
                    if name == "xtensa-esp32s3-elf-gcc" {
                        return true;
                    }
                }
            }
        }

        false
    }

    walk(Path::new(dir))
}

fn setup_environment() {
    use std::fs::{read_to_string, OpenOptions};
    use std::io::Write;

    let home = std::env::var("HOME").unwrap();

    let bashrc = format!("{home}/.bashrc");

    let line = format!(". {home}/export-esp.sh");

    let current = read_to_string(&bashrc).unwrap_or_default();

    if current.contains(&line) {
        println!("✓ export-esp.sh");
        return;
    }

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&bashrc)
        .unwrap();

    writeln!(file).unwrap();
    writeln!(file, "# J-BOT").unwrap();
    writeln!(file, "{}", line).unwrap();

    println!("✓ export-esp.sh");
}

#[cfg(unix)]
fn create_symlinks() {
    use std::{
        fs,
        os::unix::fs::symlink,
    };

    let home = std::env::var("HOME").unwrap();

    let cargo_bin = format!("{home}/.cargo/bin");

    let tools = [
        "cargo",
        "cargo-generate",
        "espflash",
        "espup",
        "esp-generate",
        "ldproxy",
        "rustc",
        "rustup",
    ];

    for tool in tools {

        let src = format!("{cargo_bin}/{tool}");

        let dst = format!("tools/bin/{tool}");

        if !Path::new(&src).exists() {
            continue;
        }

        // Remove old file/symlink if it exists
        let _ = fs::remove_file(&dst);

        // Create fresh symlink
        let _ = symlink(&src, &dst);
    }

    println!("✓ tools/bin updated");
}
fn check_tools() {
    check_dir("tools");
    check_dir("tools/bin");
}

fn check_dir(path: &str) {
    if Path::new(path).exists() {
        println!("✓ {}", path);
    } else {
        println!("✗ {}", path);
    }
}

fn check(cmd: &str) {
    let ok = Command::new("which")
        .arg(cmd)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    if ok {
        println!("✓ {}", cmd);
    } else {
        println!("✗ {}", cmd);
    }
}

fn command_exists(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn ensure_tool(tool: &str, install: &[&str]) {
    if command_exists(tool) {
        println!("✓ {}", tool);
        return;
    }

    println!("Installing {}...", tool);

    let status = Command::new(install[0])
        .args(&install[1..])
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("✓ {}", tool);
        }
        _ => {
            println!("✗ {}", tool);
        }
    }
}

fn verify_board() {
    let entries = fs::read_dir("/dev").unwrap();

    let mut found = false;

    for entry in entries.flatten() {
        let name = entry.file_name();
        let name = name.to_string_lossy();

        if name.starts_with("ttyACM") || name.starts_with("ttyUSB") {
            println!("✓ ESP Device : /dev/{}", name);
            found = true;
        }
    }

    if !found {
        println!("✗ No ESP Board Connected");
    }
}

fn check_path() {
    let home = std::env::var("HOME").unwrap();

    let cargo_bin = format!("{home}/.cargo/bin");

    let path = std::env::var("PATH").unwrap_or_default();

    if path.contains(&cargo_bin) {
        println!("✓ PATH");
    } else {
        println!("✗ PATH");
    }
}

fn version(cmd: &str, arg: &str) {
    if let Ok(out) = Command::new(cmd).arg(arg).output() {
        if out.status.success() {
            let text = String::from_utf8_lossy(&out.stdout);
            println!("✓ {}", text.trim());
            return;
        }
    }

    println!("✗ {}", cmd);
}
