
use std::env;

pub struct Config{
    pub file: String,
}

// total hack way to do this
pub fn parse_args() -> Config {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Invalid args");
        std::process::exit(1);
    }

    Config { 
        file:  args[1].to_owned(),
    }
}