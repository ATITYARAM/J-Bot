use std::{
    fs,
    net::TcpListener,
    process::Command,
};

pub fn run() {
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("             Spectate Doctor");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    let ok = check();

    println!();

    if ok {
        println!("Result : PASS");
    } else {
        println!("Result : FAIL");
    }

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
}

pub fn check() -> bool {
    let mut ok = true;

    println!("System");
    println!("──────");

    if !check_hostname() {
        ok = false;
    }

    println!();

    println!("Networking");
    println!("──────────");

    if !check_port() {
        ok = false;
    }

    println!();

    println!("Optional Features");
    println!("─────────────────");

    check_avahi_installed();
    check_avahi_running();
    check_mdns();
    check_nss();

    ok
}

fn check_hostname() -> bool {
    match hostname::get() {
        Ok(name) => {
            println!("✓ Hostname              {}", name.to_string_lossy());
            true
        }
        Err(_) => {
            println!("✗ Hostname");
            false
        }
    }
}

fn check_port() -> bool {
    match TcpListener::bind("0.0.0.0:4999") {
        Ok(_) => {
            println!("✓ Port 4999 Available");
            true
        }
        Err(_) => {
            println!("✗ Port 4999 Already In Use");
            false
        }
    }
}

fn check_avahi_installed() {
    let found = Command::new("which")
        .arg("avahi-daemon")
        .status()
        .map(|s| s.success())
        .unwrap_or(false);

    if found {
        println!("✓ Avahi Installed");
    } else {
        println!("⚠ Avahi Not Installed (mDNS disabled)");
        println!("    Ubuntu : sudo apt install avahi-daemon");
        println!("    Fedora : sudo dnf install avahi");
        println!("    RHEL   : sudo dnf install avahi");
    }
}

fn check_avahi_running() {
    let running = Command::new("systemctl")
        .args(["is-active", "--quiet", "avahi-daemon"])
        .status()
        .map(|s| s.success())
        .unwrap_or(false);

    if running {
        println!("✓ Avahi Running");
    } else {
        println!("⚠ Avahi Not Running (mDNS disabled)");
        println!("    sudo systemctl enable --now avahi-daemon");
    }
}

fn check_mdns() {
    let rpm = Command::new("rpm")
        .args(["-q", "nss-mdns"])
        .status()
        .map(|s| s.success())
        .unwrap_or(false);

    let dpkg = Command::new("dpkg")
        .args(["-s", "libnss-mdns"])
        .status()
        .map(|s| s.success())
        .unwrap_or(false);

    let installed = rpm || dpkg;

    if installed {
        println!("✓ nss-mdns Installed");
    } else {
        println!("⚠ nss-mdns Not Installed (mDNS disabled)");
        println!("    Ubuntu : sudo apt install libnss-mdns");
        println!("    Fedora : sudo dnf install nss-mdns");
        println!("    RHEL   : sudo dnf install nss-mdns");
    }
}

fn check_nss() {
    let contents = fs::read_to_string("/etc/nsswitch.conf")
        .unwrap_or_default();

    if contents.contains("mdns") {
        println!("✓ NSS mDNS Configured");
    } else {
        println!("⚠ NSS mDNS Not Configured");
        println!("    Edit /etc/nsswitch.conf");
        println!("    hosts: files mdns4_minimal [NOTFOUND=return] dns myhostname");
    }
}
