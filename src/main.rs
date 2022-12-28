use std::io;
use std::num::ParseIntError;

fn main() {
    println!("Guess the number: ");

    let mut read_guess = String::new();

    io::stdin().read_line(&mut read_guess).expect("Error reading from the command line.");

    let guess: u32 = match read_guess.trim().parse::<u32>() {

        Ok(num) => num,
        Err(_) => 0,
    
    };

    println!("You entered: {}.", guess);

}
