use std::env;
use std::error::Error;
use std::fs;
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let res = if config.case_insensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    for item in res {
        println!("{}", item);
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_insensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("没有输入查询字符串名称"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("没有输入文件名"),
        };
        let case_insensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            query,
            filename,
            case_insensitive,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_result() {
        let query = "🚀";
        let contents = "\
        Rust:__rust_force_expr!\n
🚀
        导弹
        ";

        assert_eq!(vec!["🚀"], search(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";

        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

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

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contains: &'a str) -> Vec<&'a str> {
    contains
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}
