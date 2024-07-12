use dialoguer::Select;

use rasterizer::subcommands::{gradient, sequence, variants};

fn main() {
    let subcommands = vec!["gradient", "sequence", "variants"];

    let selection = Select::new()
        .with_prompt("Choose generation type")
        .items(&subcommands)
        .interact()
        .unwrap();

    match subcommands[selection] {
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

fn help() {
    println!("===== HELP =====");
    println!("subcommands:");
    println!("  > gradient");
    println!("  > sequence");
    println!("  > variants");
}