use std::env;
use std::fs;
use std::process;
use std::error::Error;

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn parse_args(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not Enough Arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>>{
    println!("Query is: {:?}", config.query);

    let contents = fs::read_to_string(&config.file_path)?;
    println!("In file {}", config.file_path);

    println!("With text:\n{contents}");
    Ok(())
}

// this is commend
fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::parse_args(&args).unwrap_or_else(|err| {
        println!("Problem Parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application Error {e}");
        process::exit(1);
    }
}
