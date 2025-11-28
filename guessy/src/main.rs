// standard library type shit
use std::cmp::Ordering;
use std::io;

// download rand crate and use the random number
use rand::Rng;
fn main() {
    // loop, does a loop tf do you think it does bruh
    loop {
        println!("Guess your number and input your guess");
        /*
        string::new is an empty string

        'let' sets a variable immutable by default ie "let pies = 5;"
        'let mut' sets a mutable variable ie "let mut pies = 5;"
        rng = random number, gen_range = range duh(

        so the book said "thread_rng" now it's just rng, and gen_range iss now random_range, thanks rust-analyzer

        )
        */
        let secret = rand::rng().random_range(1..=100);
        let mut guess = String::new();

        // input, and using the lib
        io::stdin()
            // input and store to guess and it's Mutable *sparkles* and the '&' is a refernece ie can read without making a shit tonne in memory or something
            .read_line(&mut guess)
            .expect("failed to read line");
        // guess is now a unsigned 32bit number, trim = remove whitespace and \n, parse to change type

        // Handling Invalid Input blah blah read book
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // {guess} feels like the same as f-string in python ;3
        println!("You guessed: {guess}");

        match guess.cmp(&secret) {
            Ordering::Less => println!("small dimwit"),
            Ordering::Greater => println!("big dimwit"),
            Ordering::Equal => {
                println!("correct dimwit");
                break;
            }
        }
    }
}
