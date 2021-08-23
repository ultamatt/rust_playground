use std::{ io, env, process }; //Using two standard libs, so using short form syntax

mod player;
use player::Player;

mod game;
use game::Game;

fn main() {
    println!("Please enter a name for Player One");

    let mut daPlayerOne: Player;
    loop {
        let mut p1:String = String::new(); //Define p1 as a new String

        io::stdin()
            .read_line(&mut p1) //Read into the p1 variable, referencing the mutable address
            .expect("Failed to read line");

        daPlayerOne = match Player::new(p1.trim(), true) {
            Ok(myPlayer) => myPlayer,
            Err(error) => {
                println!("Please enter a valid string as your name");
                continue;
            },
        };
        break;
    }
    println!("Ready Player One, AKA {}", daPlayerOne.name);
}
