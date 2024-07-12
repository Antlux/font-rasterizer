use std::env::args;

fn main() {
    let mut args = args();
    if let Some(subcommand) = args.nth(1) {
        match subcommand.as_str() {
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