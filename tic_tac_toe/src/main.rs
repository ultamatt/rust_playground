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

    println!("Ready to play {} and {}?", daPlayerOne.name, daPlayerTwo.name);

    let mut daGame: Game = Game::new().unwrap();


    loop {
        daGame.draw();
        let da_move = get_new_move();
        daGame.claim(da_move.0, da_move.1);
        daGame.draw();

        if(daGame.is_finished()){
            break;
        }
    }
}

fn get_new_move() -> (u8, u8) {
    let mut tac_move:(u8, u8) = (0, 0);
    loop {
        let mut da_row:String = String::new(); //Define da_row as a new String
        println!("What row would you like?");
        io::stdin()
            .read_line(&mut da_row) //Read into the da_row variable, referencing the mutable address
            .expect("Failed to read line");

        let row:u8 = match da_row.trim().parse() {
            Ok(myRow) => myRow,
            Err(error) => {
                println!("Please enter a valid row!");
                continue;
            },
        };

        let mut da_column:String = String::new(); //Define da_column as a new String
        println!("What column would you like?");
        io::stdin()
            .read_line(&mut da_column) //Read into the da_column variable, referencing the mutable address
            .expect("Failed to read line");

        let column:u8 = match da_column.trim().parse() {
            Ok(myColumn) => myColumn,
            Err(error) => {
                println!("Please enter a valid column!");
                continue;
            },
        };
        tac_move = (row, column);
        break;
    }
    println!("Gonna tac_move to {:?}", tac_move);
    tac_move
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
