use std::env;
use std::process;

use minigrep;


fn main() {
    let args: Vec<String> = env::args().collect();
    let config = minigrep::parse_config(&args).unwrap_or_else(|err| {
        println!("{err}");
        process::exit(1);
    });
    if let Err(e) = minigrep::run(config) {
        println!("{e}");
        process::exit(1);
    }
}   