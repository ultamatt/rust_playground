use std::{ fs, env }; //Using two standard libs, so using short form syntax
use std::error::Error;

//A structure to hold the query we wish to search said file for.
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

//Function implementation for this struct. Similar to interface classes in Java
impl Config {
    //Takes a reference to an array of strings
    pub fn new (mut args: env::Args) -> Result<Config, &'static str> {
        // if args.len() < 3 { //No parenthesis needed.
        //     return Err("Please pass a string to find and a file to find it on.")
        // }
        // let query = args[1].clone();
        // let filename = args[2].clone();

        args.next(); //We are assured to have the name of this program, no need to check

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        println!("Case sense {}", case_sensitive);

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

// pub fn search<'a>(
//     query: &str,
//     contents: &'a str
// ) -> Vec<&'a str> {
//     println!("Running normal search");
//     let mut results = Vec::new();
//     for line in contents.lines() {
//         if line.contains(query) {
//             // do something with line
//             results.push(line);
//         }
//     }
//
//     results
// }

pub fn search<'a>(
    query: &str,
    contents: &'a str
) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    println!("Running insensitive search");
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
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

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
