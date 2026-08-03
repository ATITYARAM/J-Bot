use std::process::{Command, Stdio};

pub fn run() {
    println!("=========================");
    println!("ROS 2 Setup");
    println!("=========================\n");

    apt_update();

    install_prerequisites();

    add_repository();

    apt_update();

    install_ros_base();

    install_ros_tools();

    println!();
    println!("ROS Base Installed");
}

fn apt_update() {
    let _ = Command::new("sudo")
        .args(["apt", "update"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();
}

fn install_prerequisites() {
    println!("Installing prerequisites...");

    let _ = Command::new("sudo")
        .args([
            "apt",
            "install",
            "-y",
            "curl",
            "gnupg",
            "lsb-release",
            "software-properties-common",
            "locales",
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();

    let _ = Command::new("sudo")
        .args(["locale-gen", "en_US.UTF-8"])
        .status();

    let _ = Command::new("sudo")
        .args([
            "update-locale",
            "LANG=en_US.UTF-8",
            "LC_ALL=en_US.UTF-8",
        ])
        .status();

    println!("✓ Prerequisites");
}

fn add_repository() {
    println!("Adding ROS Repository...");

    let cmd = r#"
curl -sSL https://raw.githubusercontent.com/ros/rosdistro/master/ros.key |
sudo gpg --dearmor -o /usr/share/keyrings/ros-archive-keyring.gpg

echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/ros-archive-keyring.gpg] http://packages.ros.org/ros2/ubuntu $(. /etc/os-release && echo $UBUNTU_CODENAME) main" |
sudo tee /etc/apt/sources.list.d/ros2.list >/dev/null
"#;

    let _ = Command::new("bash")
        .arg("-c")
        .arg(cmd)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();

    println!("✓ Repository");
}

fn install_ros_base() {
    println!("Installing ROS Base...");

    let _ = Command::new("sudo")
        .args([
            "apt",
            "install",
            "-y",
            "ros-jazzy-ros-base",
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();

    println!("✓ ros-base");
}

fn install_ros_tools() {
    println!("Installing ROS tools...");

    let _ = Command::new("sudo")
        .args([
            "apt",
            "install",
            "-y",
            "python3-colcon-common-extensions",
            "python3-vcstool",
            "python3-rosdep",
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();

    println!("✓ ROS tools");
}
