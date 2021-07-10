use std::io; //Use the standard io library
use std::cmp::Ordering;
use rand::Rng;

pub struct Guess {
    value: i32
}

impl Guess {
    pub fn new (value:i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess values should be between 1 and 100! We got {}", value);
            return Guess { value };
        }

        return Guess { value };
    }

    pub fn value (&self) -> i32 {
        self.value
    }
}

fn main() {
    println!("Guess the Number between 1 and 100!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    //println!("The secret number is: {}", secret_number);

    let mut guesses = 0;
    loop {
        println!("Please input your guess.");

        //mut means mutable, which means this variable can be changed after initialization
        let mut guess = String::new(); //Define guess as a new String

        io::stdin()
            .read_line(&mut guess) //Read into the guess variable, referencing the mutable address
            .expect("Failed to read line");

        let guess: Guess = match guess.trim().parse() {
            Ok(num) => Guess::new(num),
            Err(_) => continue,
        };

        println!("You guessed: {}", guess.value());
        guesses = guesses + 1;

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! It took {} guesses!", guesses);
                break;
            }
        }
    }
}
