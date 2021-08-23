use std::{ fs, env }; //Using two standard libs, so using short form syntax
use std::error::Error;

///A structure to hold Information on the player
pub struct Game {
    pub over: bool, //true for O, false for X
    pub board: Vec< Vec <String> >
}

///Function implementation for this struct. Similar to interface classes in Java
impl Game {
    //Takes a reference to an array of strings
    pub fn new () -> Result<Game,  &'static str> {
        // if args.len() < 3 { //No parenthesis needed.
        //     return Err("Please pass a string to find and a file to find it on.")
        // }
        // let query = args[1].clone();
        // let filename = args[2].clone();

        // args.next(); //We are assured to have the name of this program, no need to check
        //
        // let query = match args.next() {
        //     Some(arg) => arg,
        //     None => return Err("Didn't get a query string"),
        // };
        //
        // let filename = match args.next() {
        //     Some(arg) => arg,
        //     None => return Err("Didn't get a file name"),
        // };
        //
        // let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        //println!("Case sense {}", case_sensitive);

        Ok(Game {
            over:false,
            board: vec![
                vec![String::from(""), String::from(""), String::from("")],
                vec![String::from(""), String::from(""), String::from("")],
                vec![String::from(""), String::from(""), String::from("")]
            ]
        })
    }

    pub fn draw (&self) {
        println!("Current Game State:\r\n");
        println!("   1   2  3 ");
        println!("  ----------");
        let mut row_num = 0;
        for x in &self.board {
            row_num = row_num + 1;
            println!("{} | {} | {} | {} |", row_num, x[0], x[1], x[2]);
            println!("  ----------");
        }
    }

    pub fn claim (&mut self, row:u8, column:u8) {
        // Ugh, fucking vector of vectors.
        println!("You want r{}, c{}", row, column);
        for (dis_row_num, dis_row_val) in self.board.iter_mut().enumerate() {
            for (dis_column_num, dis_column_val) in dis_row_val.iter_mut().enumerate() {
                if(dis_row_num as u8 == row && dis_column_num as u8 == column){
                    *dis_column_val = String::from("X");
                }
            }
        }
    }

    pub fn is_finished (&self) -> bool{
        true
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
//
// pub fn search<'a>(
//     query: &str,
//     contents: &'a str
// ) -> Vec<&'a str> {
//     contents
//         .lines()
//         .filter(|line| line.contains(query))
//         .collect()
// }
//
// pub fn search_case_insensitive<'a>(
//     query: &str,
//     contents: &'a str,
// ) -> Vec<&'a str> {
//     println!("Running insensitive search");
//     let query = query.to_lowercase();
//     let mut results = Vec::new();
//
//     for line in contents.lines() {
//         if line.to_lowercase().contains(&query) {
//             results.push(line);
//         }
//     }
//
//     results
// }
//
// pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
//     let contents = fs::read_to_string(config.filename)?;
//
//     let results = if config.case_sensitive {
//         search(&config.query, &contents)
//     } else {
//         search_case_insensitive(&config.query, &contents)
//     };
//
//     for line in results {
//         println!("{}", line);
//     }
//
//     Ok(())
// }
// //
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn one_player_basic() {
//         let test_name = "Testy";
//         let myGame = Game::new(test_name, true).unwrap();
//         assert_eq!(test_name, myGame.name);
//         assert_eq!(0, myGame.score);
//     }
//
//     #[test]
//     fn two_player_basic() {
//         let test_name_one = "Testy One";
//         let test_name_two = "Testy Two";
//         let myGameOne = Game::new(test_name_one, true).unwrap();
//         let myGameTwo = Game::new(test_name_two, false).unwrap();
//         assert_eq!(test_name_one, myGameOne.name);
//         assert_eq!(0, myGameOne.score);
//
//         assert_eq!(test_name_two, myGameTwo.name);
//         assert_eq!(0, myGameTwo.score);
//
//         assert_ne!(myGameOne.name, myGameTwo.name);
//     }
// }
