use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    let mut guesses: u32 = 0;
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("What's the number: ");
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        guesses += 1;

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You win!");
                break;
            },
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
        }
    }

    println!("You tried {guesses} times");
}
