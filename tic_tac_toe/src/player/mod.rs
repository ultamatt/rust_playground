use std::{ fs, env }; //Using two standard libs, so using short form syntax
use std::error::Error;

///A structure to hold Information on the player
pub struct Player {
    pub name: String,
    pub score: u32,
    pub x_or_o: bool, //true for O, false for X
}

///Function implementation for this struct. Similar to interface classes in Java
impl Player {
    //Takes a reference to an array of strings
    pub fn new (name: &str, x_or_o: bool) -> Result<Player,  &'static str> {
        if((*name).is_empty()) {
            return Err("Please pass a player's name which is not blank");
        }

        Ok(Player {
            name:String::from(name),
            score:0,
            x_or_o:x_or_o,
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
//
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_player_basic() {
        let test_name = "Testy";
        let myPlayer = Player::new(test_name, true).unwrap();
        assert_eq!(test_name, myPlayer.name);
        assert_eq!(0, myPlayer.score);
    }

    #[test]
    fn two_player_basic() {
        let test_name_one = "Testy One";
        let test_name_two = "Testy Two";
        let myPlayerOne = Player::new(test_name_one, true).unwrap();
        let myPlayerTwo = Player::new(test_name_two, false).unwrap();
        assert_eq!(test_name_one, myPlayerOne.name);
        assert_eq!(0, myPlayerOne.score);

        assert_eq!(test_name_two, myPlayerTwo.name);
        assert_eq!(0, myPlayerTwo.score);

        assert_ne!(myPlayerOne.name, myPlayerTwo.name);
    }
}
