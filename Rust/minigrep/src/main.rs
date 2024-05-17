use std::env;
use std::process;
// Grep Logic here
use minigrep::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Promblem parsing arguments: {err}");
        process::exit(1);
    });
    if let Err(e) = config.run() {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
