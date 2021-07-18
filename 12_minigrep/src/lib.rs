use std::{ fs }; //Using two standard libs, so using short form syntax
use std::error::Error;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new (args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 { //No parenthesis needed.
            return Err("Please pass a string to find and a file to find it on.")
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}
