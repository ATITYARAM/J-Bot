use std::{
    fs,
    path::Path,
    process::Command,
};

pub fn run() {
    println!("=========================");
    println!("J-BOT Environment");
    println!("=========================\n");

    check_os();
    check_rust();
    check_esp();
    check_xtensa();
    check_tools();

    println!();

    println!("Environment Ready");
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
    check("rustc");
    check("rustup");
}

fn check_esp() {
    check("espup");
    check("espflash");
    check("esp-generate");
    check("cargo-generate");
    check("ldproxy");
}

fn check_xtensa() {
    let home = std::env::var("HOME").unwrap_or_default();

    let gcc = format!(
        "{}/.rustup/toolchains/esp/xtensa-esp-elf/esp-15.2.0_20250920/xtensa-esp-elf/bin/xtensa-esp32s3-elf-gcc",
        home
    );

    if Path::new(&gcc).exists() {
        println!("✓ xtensa-esp32s3-elf-gcc");
    } else {
        println!("✗ xtensa-esp32s3-elf-gcc");
    }
}

fn check_tools() {
    if Path::new("tools").exists() {
        println!("✓ tools/");
    } else {
        println!("✗ tools/");
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
