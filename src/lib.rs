use std::error::Error;
use std::fs;
use std::task::Context;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{    
    let contents = fs::read_to_string(config.file_path)?;

    //println!("With text:\n{contents}");
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    }
    else {
        search(&config.query, &contents)
    };


    for line in results
    {
        println!("{line}");
    }


    Ok(()) //()叫做一个单元类型,因为我们的程序无需任何返回值,但为了满足Result<T,E>的要求,这里返回一个Ok(())的类型.
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool, //新增配置项,用于控制是否开启大小写敏感.
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config,&'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case, })
    } 
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."],search(query,contents));
    }


    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
    
        assert_eq!(vec!["Rust:","Trust me."],search_case_insensitive(query, contents));
    
    }
}

pub fn search<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    
    for line in contents.lines() {
        if line.contains(query)
        {
            results.push(line);
        }
    }
    
    results
}

pub fn search_case_insensitive<'a> (query: &str, contents: &'a str) -> Vec<&'a str>
{
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query)
        {
            results.push(line);
        }
    }
    results
}