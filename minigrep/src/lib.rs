use std::error::Error;
use std::fs;
use std::env;

const BASE_DIR: &'static str = r"C:\Users\zqm\Desktop\all_test\rust_study\minigrep\input\";  


#[derive(Debug)]
pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}


pub fn run (config: Config) ->Result<(), Box<dyn Error>> {
    // Box<dyn Error> 包裹？的返回错误，动态大小
    println!("{:?}", config);
    let contents = fs::read_to_string(config.file_path)?;
    println!("with text: \n{contents}");
    let result = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    for i in result {
        println!("{i}");
    }
    Ok(())
}

pub fn parse_config(args: &[String]) ->Result<Config, &'static str> {
    if args.len() < 3 {
        return Err("not enough arguments");
    }
    let file_path = BASE_DIR.to_string() + &args[1];
    let query = args[2].clone();
    let ignore_case = env::var("IGNORE_CASE").is_ok(); // env::var 或者环境变量，非win可以如此设置 IGNORE_CASE=1 cargo run 
   Ok( Config { query: query, file_path: file_path, ignore_case: ignore_case })
}

pub fn search<'a>(query: &str, content: &'a str) ->Vec<&'a str> {
    let ret: Vec<&'a str> = content.lines().filter(|x| x.contains(query)).collect();
    ret
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) ->Vec<&'a str> {
    let ret: Vec<&'a str> = content.lines().filter(|x| x.to_lowercase().contains(&query.to_lowercase())).collect();
    ret
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case_sensitive() {
        let query = "pig";
        let contents = "bear
        pig
        napple";
        // println!("{:?}", search(query, contents));
        assert_eq!(vec!["pig"], search(query, contents));
    }

    #[test]
    fn case_insensitive () {
        let query = "pig";
        let contents = "pig
        Pig123
        apple";
        // println!("{:?}", search(query, contents));
        assert_eq!(vec!["pig, Pig123"], search_case_insensitive(query, contents));
    }
}