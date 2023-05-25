use std::{io, cmp::Ordering};
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let random_num: u32 = rand::thread_rng()
        .gen_range(1..=100);

    loop {
        println!("Please input your guess: ");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(res) => res,
            Err(_) => continue
        };

        match guess.cmp(&random_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You win!");
                break
            }
        }
    };
}
