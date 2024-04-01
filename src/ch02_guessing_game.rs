use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::str::FromStr;

pub fn run() {
    let secret = gen_secret();
    println!("Secret: {secret}");

    loop {
        println!("Please enter you guess: ");
        let guess: i32 = match read_guess() {
            Ok(num) => num,
            Err(_) => continue,
        };

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

fn read_guess<F: FromStr>() -> Result<F, F::Err> {
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read guess!");

    guess.trim().parse()
}

fn gen_secret() -> i32 {
    rand::thread_rng().gen_range(1..=10)
}
