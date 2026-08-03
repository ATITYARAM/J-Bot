use std::{
    fs,
    process::{Command, Stdio},
};

pub fn run() {
    header();

    let ubuntu = detect_ubuntu();
    let ros = ros_distro(&ubuntu);

    print_detected(&ubuntu, &ros);

    install_packages();

    if !repository_exists() {
        add_ros_repository();
        apt_update();
    }  

    if ros_installed(&ros) {
        println!("✓ ros-base");
    } else {
        install_ros(&ros);
    }
    install_tools();

    init_rosdep();

    update_bashrc(&ros);

    create_tools();

    create_symlinks();

    verify();

    versions();

    footer();
    
    if !internet_available() {
        println!("No internet connection.");
        return;
    }
}

fn versions() {
    version("ros2", "--version");
    version("colcon", "--version");
    version("rosdep", "--version");
}

fn version(cmd: &str, arg: &str) {
    if let Ok(out) = Command::new(cmd).arg(arg).output() {
        if out.status.success() {
            let text = String::from_utf8_lossy(&out.stdout);

            if let Some(line) = text.lines().next() {
                println!("  {}", line);
            }
        }
    }
}

fn internet_available() -> bool {
    execute(
        "ping",
        &[
            "-c",
            "1",
            "packages.ros.org",
        ],
    )
}

fn repository_exists() -> bool {
    std::path::Path::new("/etc/apt/sources.list.d/ros2.list").exists()
}

fn ros_installed(ros: &str) -> bool {
    let path = format!("/opt/ros/{}", ros);
    std::path::Path::new(&path).exists()
}

fn verify_bashrc(ros: &str) {
    let bashrc = format!(
        "{}/.bashrc",
        std::env::var("HOME").unwrap()
    );

    let txt = fs::read_to_string(bashrc).unwrap_or_default();

    let line = format!(
        "source /opt/ros/{}/setup.bash",
        ros
    );

    if txt.contains(&line) {
        println!("✓ ~/.bashrc");
    } else {
        println!("✗ ~/.bashrc");
    }
}

fn detect_ubuntu() -> String {
    let os = fs::read_to_string("/etc/os-release").unwrap_or_default();

    for line in os.lines() {
        if line.starts_with("VERSION_ID=") {
            return line
                .replace("VERSION_ID=", "")
                .replace('"', "");
        }
    }

    String::new()
}

fn apt_update() {
    println!("• apt update");

    if execute("sudo", &["apt", "update"]) {
        println!("✓ apt update");
    } else {
        println!("✗ apt update");
    }
}

fn install_packages() {
    println!("• Installing packages");

    execute(
        "sudo",
        &[
            "apt",
            "install",
            "-y",
            "curl",
            "gnupg",
            "lsb-release",
            "software-properties-common",
            "locales",
        ],
    );

    execute(
        "sudo",
        &[
            "locale-gen",
            "en_US",
            "en_US.UTF-8",
        ],
    );

    execute(
        "sudo",
        &[
            "update-locale",
            "LANG=en_US.UTF-8",
            "LC_ALL=en_US.UTF-8",
        ],
    );

    println!("✓ Packages");
}

fn add_ros_repository() {
    println!("• Adding ROS Repository");

    let cmd = r#"
        curl -sSL https://raw.githubusercontent.com/ros/rosdistro/master/ros.key \
        | sudo gpg --dearmor -o /usr/share/keyrings/ros-archive-keyring.gpg

        echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/ros-archive-keyring.gpg] http://packages.ros.org/ros2/ubuntu $(. /etc/os-release && echo $UBUNTU_CODENAME) main" \
        | sudo tee /etc/apt/sources.list.d/ros2.list >/dev/null
        "#;

    if execute_shell(cmd) {
        println!("✓ ROS Repository");
    } else {
        println!("✗ ROS Repository");
    }
}

fn execute_shell(script: &str) -> bool {
    Command::new("bash")
        .arg("-c")
        .arg(script)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();

    match chars.next() {
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

fn ros_distro(ubuntu: &str) -> &'static str {
    match ubuntu {
        "22.04" => "humble",
        "24.04" => "jazzy",
        _ => "jazzy",
    }
}

fn install_ros(ros: &str) {
    println!("• Installing ros-base");

    let pkg = format!("ros-{}-ros-base", ros);

    if execute(
        "sudo",
        &[
            "apt",
            "install",
            "-y",
            &pkg,
        ],
    ) {
        println!("✓ ros-base");
    } else {
        println!("✗ ros-base");
    }
}

fn install_tools() {
    println!("• Installing ROS tools");

    if execute(
        "sudo",
        &[
            "apt",
            "install",
            "-y",
            "python3-rosdep",
            "python3-colcon-common-extensions",
            "python3-vcstool",
            "build-essential",
            "cmake",
            "git",
        ],
    ) {
        println!("✓ ROS tools");
    } else {
        println!("✗ ROS tools");
    }
}

fn init_rosdep() {
    println!("• rosdep");

    let init = if std::path::Path::new("/etc/ros/rosdep/sources.list.d/20-default.list").exists() {
        true
    } 
    else {
        execute("sudo", &["rosdep", "init"])
    };

    let update = execute("rosdep", &["update"]);

    if init && update {
        println!("✓ rosdep");
    } else {
        println!("✗ rosdep");
    }
}

fn update_bashrc(ros: &str) {
    use std::{
        fs::{read_to_string, OpenOptions},
        io::Write,
    };

    let home = std::env::var("HOME").unwrap();
    let bashrc = format!("{}/.bashrc", home);

    let mut current = read_to_string(&bashrc).unwrap_or_default();

    let ros_line = format!("source /opt/ros/{}/setup.bash", ros);

    let lines = [
        String::from("# J-BOT ROS2"),
        ros_line,
    ];

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&bashrc)
        .unwrap();

    for line in &lines {
        if !current.contains(line) {
            writeln!(file, "{}", line).unwrap();
            current.push_str(line);
            current.push('\n');
        }
    }

    println!("✓ ~/.bashrc");
}

#[cfg(unix)]
fn create_symlinks() {
    use std::{
        os::unix::fs::symlink,
        path::Path,
    };

    let binaries = [
        "/usr/bin/ros2",
        "/usr/bin/colcon",
        "/usr/bin/rosdep",
        "/usr/bin/vcs",
    ];

    for src in binaries {
        if !Path::new(src).exists() {
            continue;
        }

        let name = Path::new(src)
            .file_name()
            .unwrap()
            .to_string_lossy();

        let dst = format!("tools/bin/{}", name);

        if Path::new(&dst).exists() {
        let _ = fs::remove_file(&dst);
    }

    let _ = symlink(src, &dst);

    }

    println!("✓ tools/bin");
}

fn verify() {
    verify_tool("ros2", "--help");
    verify_tool("colcon", "--version");
    verify_tool("rosdep", "--version");
    verify_tool("vcs", "--help");
}

fn verify_tool(cmd: &str, arg: &str) {
    let output = Command::new(cmd)
        .arg(arg)
        .output();

    match output {
        Ok(o) if o.status.success() => {
            println!("✓ {}", cmd);
        }

        _ => {
            println!("✗ {}", cmd);
        }
    }
}

fn execute(program: &str, args: &[&str]) -> bool {
    Command::new(program)
        .args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn create_tools() {
    let dirs = [
        "tools",
        "tools/bin",
    ];

    for dir in dirs {
        let _ = fs::create_dir_all(dir);
    }

    println!("✓ tools/");
}

fn header() {
    println!("=========================");
    println!("ROS 2 Setup");
    println!("=========================");
    println!();
}

fn print_detected(ubuntu: &str, ros: &str) {
    println!("Detected:");
    println!("Ubuntu {}", ubuntu);
    println!("ROS 2 {}", capitalize(ros));
    println!();
}

fn footer() {
    println!();
    println!("=========================");
    println!("ROS Ready");
    println!("=========================");
}
