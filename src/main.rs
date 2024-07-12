use std::env::args;

use rasterizer::subcommands::{gradient, sequence, variants};

fn main() {
    let mut args = args();
    if let Some(subcommand) = args.nth(1) {
        match subcommand.as_str() {
            "gradient" => gradient(),
            "sequence" => sequence(),
            "variants" => variants(),

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
    println!("  > nothing")
}