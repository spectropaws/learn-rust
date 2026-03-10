use rand::{RngExt};
use std::io::{self, Write};

fn main() {
    let mut rng = rand::rng();
    let rand_num: i32 = rng.random_range(1..=100);
    let mut tries = 0;
    println!("Guess the Number!");
    
    loop {
        tries += 1;
        let num: i32 = match input_msg("Input number: ") {
            Ok(n) => n,
            Err(e) => { 
                println!("{}", e);
                return;
            }
        };

        if num > rand_num { println!("Too big!") }
        else if num < rand_num { println!("Too small!") }
        else {
            println!("Correct! You guessed it in {} tries", tries);
            return;
        }
    }
}

fn input_msg(message: &str) -> Result<i32, String> {
    print!("{}", message);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {},
        Err(_) => return Err("Error reading input!".to_string()),
    }

    input.trim().parse()
        .map_err(|_| "Invalid number!".to_string())
}
