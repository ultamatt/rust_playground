use std::io; //Use the standard io library
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the Number between 1 and 100!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    //println!("The secret number is: {}", secret_number);


    loop {
        println!("Please input your guess.");

        //mut means mutable, which means this variable can be changed after initialization
        let mut guess = String::new(); //Define guess as a new String

        io::stdin()
            .read_line(&mut guess) //Read into the guess variable, referencing the mutable address
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
