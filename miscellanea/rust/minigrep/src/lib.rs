use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    pub fn parse_args(
        mut args: impl Iterator<Item = String>
        ) -> Result<Config, &'static str> {

        // pass the file name
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("No Query Found! Need at least two argments"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("No file path Found! Need at least two argments"),
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config { query, file_path, ignore_case})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;

    let result = if config.ignore_case{
        search_no_case(&config.query, &contents)
    } else {
        search(&config.query, &contents) 
    };

    for line in result {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents 
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_no_case<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

    let query = query.to_lowercase();
    contents 
        .lines()
        .filter(|line| line.contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "Duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search_no_case(query, contents));
    }
}
