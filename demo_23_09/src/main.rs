use std::env;
use std::error::Error;
use std::fs;
use std::process;

const BASE_DIR: &'static str = r"C:\Users\zqm\Desktop\all_test\rust_study\demo_23_09\input\";

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args).unwrap_or_else(|err| {
        println!("{err}");
        process::exit(1);
    });
    if let Err(e) = run(config) {
        println!("{e}");
        process::exit(1);
    }
}   

fn run (config: Config) ->Result<(), Box<dyn Error>> {
    // Box<dyn Error> 包裹？的返回错误，动态大小
    println!("{:?}", config);
    let contents = fs::read_to_string(config.file_path)?;
    println!("with text: \n{contents}");
    Ok(())
}

#[derive(Debug)]
struct Config {
    query: String,
    file_path: String,
}

fn parse_config(args: &[String]) ->Result<Config, &'static str> {
    if args.len() < 3 {
        return Err("not enough arguments");
    }
    let file_path = BASE_DIR.to_string() + &args[1];
    let query = args[2].clone();
   Ok( Config { query: query, file_path: file_path })
}