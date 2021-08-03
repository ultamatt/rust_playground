use std::{ env, process }; //Using two standard libs, so using short form syntax
use minigrep::Config;

fn main() {
    //Collects all items passed in environment args to be collcted into a Vector of strings
    //let args: Vec <String> = env::args().collect();
    let config = Config::new(env::args()).unwrap_or_else(|err| { //Unwrapping the error if there is one
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    //println!("Searching for {}", config.query);
    //println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
