use std::env;
use std::process;
use bookup;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = bookup::Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Warning: {err}");
        process::exit(1);
    });
    
    if let Err(err) = bookup::run(config) {
        eprintln!("Error: {err}");
        process::exit(1);
    }
}
