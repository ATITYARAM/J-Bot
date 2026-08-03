mod setup;
mod ros_setup;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        match args[1].as_str() {
            "setup" => {
                setup::run();
                return;
            }

            "ros-setup" => {
                ros_setup::run();
                return;
            }

            _ => {}
        }
    }

    println!("J-BOT");
    println!();
    println!("Usage:");
    println!("    cargo run -- setup");
    println!("    cargo run -- ros-setup");
}

