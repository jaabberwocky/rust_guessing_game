use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    // mut declares a mutable variable
    // rust variables are immutable by default
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("Secret number: {}", secret_number);

    loop {
        let mut guess = String::new();
        println!("Please input your guess.");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("{}", err);
                continue;
            }
        }; // shadowing for conversion

        println!("Your guess: {}", guess);

        match handle_guess(guess, &secret_number) {
            true => break,
            false => () // "pass" in python
        }
    }
}

fn handle_guess(guess: i32, &sec_num: &i32) -> bool {
    match guess.cmp(&sec_num) {
        Ordering::Less => {
            println!("Too small!");
            return false;
        }
        Ordering::Greater => {
            println!("Too big!");
            return false;
        }
        Ordering::Equal => {
            println!("You win!");
            return true;
        }
    }
}
