use std::cmp::Ordering;
use std::io;
use rand::Rng;

pub fn play() {
    let secret = gen_secret();
    println!("Secret: {secret}");

    loop {
        println!("Please enter you guess: ");
        let guess = read_guess();

        match guess.cmp(&secret) {
            Ordering::Greater => println!("Number is too high!"),
            Ordering::Less => println!("Number is too low!"),
            Ordering::Equal => {
                println!("Win!");
                break;
            }
        }
    }
}

fn read_guess() -> i32 {
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read guess!");

    guess.trim().parse().expect("Please enter number!")
}

fn gen_secret() -> i32 {
    rand::thread_rng().gen_range(1..=10)
}