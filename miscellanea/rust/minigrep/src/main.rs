use std::env;
use std::process;

use minigrep::Config;

// this is commend
fn main() {
    // let args: Vec<String> = env::args().collect();

    let config: Config = Config::parse_args(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem Parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application Error {e}");
        process::exit(1);
    }
}
