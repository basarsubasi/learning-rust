
use std::{cmp::Ordering, io::{self}};
use rand:: Rng;

fn main() {

    println!("Guess the number!");

    let secret_number:i32 = rand::thread_rng().gen_range(1..=100);


    loop {

        println!("Please enter the number you have guessed:");

        let mut guess = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("error when getting the input!");


        let guess:i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("you guessed : {}", guess);


        match guess.cmp(&secret_number) {
            Ordering::Less => println!("the input is less than the correct number"),

            Ordering::Greater => println!("the input is greater than the correct number"),

            Ordering::Equal => {
                println!("Correct! You Won!");
                break;
            }

        }




    }


}
