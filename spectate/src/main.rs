mod doctor;
mod fix;
mod api;
mod models;
mod scan;
mod api_process;
mod process;

use std::{env, net::SocketAddr};

use axum::Router;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let cmd = env::args()
        .nth(1)
        .unwrap_or_else(|| "serve".to_string());

    match cmd.as_str() {
        "serve" => serve().await,
        "doctor" => doctor::run(),
        "fix" => fix::run(),
        "help" | "--help" | "-h" => help(),
        _ => {
            eprintln!("Unknown command: {}\n", cmd);
            help();
        }
    }
}

async fn serve() {
    println!();
    println!("Running system diagnostics...");
    println!();

    if !doctor::check() {
        println!();
        println!("Issues detected.");
        println!("Attempting automatic repair...");
        println!();

        fix::run();

        println!();
        println!("Verifying repair...");
        println!();

        if !doctor::check() {
            println!();
            eprintln!("Unable to repair system.");
            std::process::exit(1);
        }
    }

    println!();
    println!("All checks passed.");
    println!("Starting Spectate...");
    println!();

    start_server().await;
}

async fn start_server() {
    use hostname::get;
    use local_ip_address::local_ip;
    use mdns_sd::{ServiceDaemon, ServiceInfo};

    let app = Router::new()
        .merge(api::router())
	.merge(api_process::router())
	.fallback_service(
            ServeDir::new("static")
                .append_index_html_on_directories(true),
        );

    let hostname = get()
        .expect("Failed to get hostname")
        .to_string_lossy()
        .into_owned();

    let ip = local_ip().expect("Unable to determine local IP");

    // -------------------------------------------------
    // Optional mDNS
    // -------------------------------------------------

    let mut mdns_enabled = false;
    let mut _mdns_daemon = None;

    match ServiceDaemon::new() {
        Ok(daemon) => {
            match ServiceInfo::new(
                "_http._tcp.local.",
                "Spectate",
                &format!("{}.local.", hostname),
                ip.to_string(),
                4999,
                None,
            ) {
                Ok(service) => {
                    if daemon.register(service).is_ok() {
                        mdns_enabled = true;
                        _mdns_daemon = Some(daemon);
                    }
                }
                Err(_) => {}
            }
        }
        Err(_) => {}
    }

    // -------------------------------------------------

    let addr = SocketAddr::from(([0, 0, 0, 0], 4999));

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("              Spectate v0.1.0");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    println!("Hostname : {}", hostname);
    println!("IP       : {}", ip);
    println!("Port     : 4999");
    println!();

    println!("URLs");
    println!("  Localhost : http://localhost:4999");
    println!("  LAN       : http://{}:4999", ip);

    if mdns_enabled {
        println!("  mDNS      : http://{}.local:4999", hostname);
    }

    println!();

    println!("Status");

    if mdns_enabled {
        println!("  ✓ mDNS enabled");
    } else {
        println!("  ⚠ mDNS unavailable (using LAN only)");
    }

    println!();
    println!("Press Ctrl+C to stop.");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to port 4999");

    axum::serve(listener, app)
        .await
        .expect("Server crashed");
}

fn help() {
    println!("Spectate v0.1.0");
    println!();
    println!("Usage:");
    println!("  spectate");
    println!("  spectate serve");
    println!("  spectate doctor");
    println!("  spectate fix");
    println!("  spectate help");
}
