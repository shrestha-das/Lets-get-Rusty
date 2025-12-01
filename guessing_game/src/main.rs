use std::io; // for handling user input
use std::cmp::Ordering; // for comparing values
use rand::Rng; // for generating random numbers

fn main() {
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1..=101);  // generates a random number between 1 and 100 inclusive
 
    loop {
    println!("Please input your guess.");
    
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => { 
        println!("You win! The number was {secret_number}.");
        break;
        },
    }
}
}
