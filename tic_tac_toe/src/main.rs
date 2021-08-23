use std::{ io, env, process }; //Using two standard libs, so using short form syntax

mod player;
use player::Player;

mod game;
use game::Game;

fn main() {
    println!("Please enter a name for Player One");
    let mut daPlayerOne: Player = get_new_player();

    println!("Please enter a name for Player Two");
    let mut daPlayerTwo: Player = get_new_player();

    println!("Ready to play {} and {}?", daPlayerOne.name, daPlayerTwo.name)
}

fn get_new_player() -> Player {
    let return_player: Player;
    loop {
        let mut da_player:String = String::new(); //Define da_player as a new String

        io::stdin()
            .read_line(&mut da_player) //Read into the da_player variable, referencing the mutable address
            .expect("Failed to read line");

        return_player = match Player::new(da_player.trim(), true) {
            Ok(myPlayer) => myPlayer,
            Err(error) => {
                println!("Please enter a valid string as your name");
                continue;
            },
        };
        break;
    }
    println!("Ready Player One, AKA {}", return_player.name);
    return_player
}
