use std::{ io, env, process }; //Using two standard libs, so using short form syntax

mod player;
use player::Player;

mod game;
use game::Game;

fn main() {
    //Get player's names and prep for game
    println!("Please enter a name for Player One");
    let mut daPlayerOne: Player = get_new_player(false);
    println!("Please enter a name for Player Two");
    let mut daPlayerTwo: Player = get_new_player(true);
    println!("{}", daPlayerOne.show_info());
    println!("{}", daPlayerTwo.show_info());
    println!("Ready to play?");
    println!("");
    println!("");

    //Get a new game and play
    let mut daGame: Game = Game::new().unwrap();
    let mut turn:u32 = 0;
    loop {
        turn += 1;
        println!("Turn {}", turn);
        daGame.draw();

        let mut x_or_o:bool = false;
        if(turn%2 == 1){
            x_or_o = true;
        }

        if(x_or_o){
            println!("{} it is your turn!", daPlayerTwo.show_info());
        } else {
            println!("{} it is your turn!", daPlayerOne.show_info());
        }

        let da_move = get_new_move();
        daGame.claim(da_move.0, da_move.1, x_or_o);

        // if(daGame.is_finished()){
        //     break;
        // }
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

        //TODO: Validate row is either 1, 2, or 3
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

        //TODO: Validate column is either 1, 2, or 3
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


fn get_new_player(x_or_o:bool) -> Player {
    let return_player: Player;
    loop {
        let mut da_player:String = String::new(); //Define da_player as a new String

        io::stdin()
            .read_line(&mut da_player) //Read into the da_player variable, referencing the mutable address
            .expect("Failed to read line");

        return_player = match Player::new(da_player.trim(), x_or_o) {
            Ok(myPlayer) => myPlayer,
            Err(error) => {
                println!("\n\n\t--> Please enter a valid string as your name");
                continue;
            },
        };
        break;
    }
    println!("Ready Player One, AKA {}", return_player.name);
    return_player
}
