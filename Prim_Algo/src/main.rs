#![allow(non_snake_case)]
use std::io;

fn main() {
    println!("Write something!");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    print!("You guessed : {}", guess);
}
