use std::io;
use rand::*;

fn main() {
    let fruits = vec!["apple", "banana", "grape"];

    println!("Guess the fruit");
    let random_number = rng().random_range(0..fruits.len());

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = guess.trim();
        println!("You guessed: {guess}");

        // println!("Random fruit is: {}", fruits[random_number]);
        if guess == fruits[random_number] {
            println!("Correct!!");
            break;
        }
        println!("Incorrect");
    }
}
