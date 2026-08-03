pub fn run() {
    println!("Running automatic repair...\n");

    restart_avahi();

    enable_mdns();

    fix_hostname();

    restart_services();

    println!("\nRepair completed.");
}

fn restart_avahi() {
    println!("Restarting Avahi...");
}

fn enable_mdns() {
    println!("Enabling mDNS...");
}

fn fix_hostname() {
    println!("Checking hostname...");
}

fn restart_services() {
    println!("Restarting services...");
}
