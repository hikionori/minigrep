use std::error::Error;
use std::{fs, env};

pub struct Config{
    pub query: String,
    pub filename: String,
    pub sensive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args)-> Result<Config, &'static str>{

        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Не получена строка запроса")
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Не получено имя файла")
        };

        let sensive = env::var("CASE_INSENSITIVE").is_err();
    
        Ok(Config{ query, filename, sensive })    
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let content = fs::read_to_string(config.filename)?;

    let result = if config.sensive {
        search(&config.query, &content)
    }
    else{
        search_case_insensitive(&config.query, &content)
    };
    for line in result{
        print!("----\n {} \n----", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str>{
    content.lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str>{
    let query = query.to_lowercase();
    let mut result = Vec::new();

    for line in content.lines(){
        if line.to_lowercase().contains(&query){
            result.push(line);
        }
    }

    result
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn case_sensivity(){
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three";
        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }

    #[test]
    fn case_insensivity(){
        let query = "rUsT";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, content)
        );
    }
}