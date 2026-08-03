mod setup;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 && args[1] == "setup" {
        setup::run();
        return;
    }

    println!("J-BOT");
    println!();
    println!("Usage:");
    println!("    cargo run -- setup");
}
