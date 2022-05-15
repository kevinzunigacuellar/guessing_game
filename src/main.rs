use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // 0. Welcome message
    println!("Welcome to the guessing game! 🎉");
    // 1. Generate a secret number
    let secret_num = rand::thread_rng().gen_range(0..=50);
    // 2. Loop until the user guesses the secret number
    loop {
        // 2.1 Promp user for a number
        println!("Please input your guess (0 - 50): ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // 2.2 Convert the string to a number
        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {guess}");
        // 2.3 Compare the number to the secret number
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("{}", "Your guess is too low!".bright_blue()),
            Ordering::Greater => println!("{}", "Your guess is too high!".yellow()),
            Ordering::Equal => {
                println!("{}", "Congratulations! You guessed right!".green());
                break;
            }
        }
    }
}
