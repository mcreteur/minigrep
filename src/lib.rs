use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        return Ok(Config {
            query,
            filename,
            case_sensitive,
        });
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(config.filename)?;

    let result = if config.case_sensitive {
        search(&config.query, &file_content)
    } else {
        search_case_insensitive(&config.query, &file_content)
    };

    for line in result {
        println!("{}", line);
    }

    return Ok(());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_a_new_config() {
        let args = vec![
            String::from("arg0"),
            String::from("arg1"),
            String::from("arg2"),
        ];

        let config = Config::new(args.into_iter()).unwrap();

        assert_eq!("arg1", config.query);
        assert_eq!("arg2", config.filename);
    }

    #[test]
    fn create_a_new_config_not_enough_args() {
        let args = vec![String::from("arg0"), String::from("arg1")];

        let config = Config::new(args.into_iter());

        assert!(config.is_err());
    }

    #[test]
    fn run_ok() {
        let config = Config {
            query: String::from("arg1"),
            filename: String::from("poem.txt"),
            case_sensitive: true,
        };

        if let Err(_) = run(config) {
            panic!();
        }
    }

    #[test]
    #[should_panic]
    fn run_should_panic() {
        let config = Config {
            query: String::from("arg1"),
            filename: String::from("inexistent file"),
            case_sensitive: true,
        };

        if let Err(_) = run(config) {
            panic!();
        }
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.
        ";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
