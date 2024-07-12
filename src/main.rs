use std::env::args;

use rasterizer::subcommands::{gradient, sequence, variants};

fn main() {
    let mut args = args();
    if let Some(subcommand) = args.nth(1) {
        match subcommand.as_str() {
            "gradient" => gradient().unwrap_or_else(|err| eprintln!("ERROR: {err}")),
            "sequence" => sequence(),
            "variants" => variants(),
            "help" => help(),
            other => {
                println!("'{other}' is not a recognized subcommand.");
                help();
            }         
        }
    }
}

fn help() {
    println!("===== HELP =====");
    println!("subcommands:");
    println!("  > gradient");
    println!("  > sequence");
    println!("  > variants");
}