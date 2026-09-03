// This is called a prelude, it brings the input/output into scope, from the standard library
use rand::Rng;
use std::cmp::Ordering;
use std::io;

// Again main entry function
fn main() {
    // Print line macro call
    println!("Guess the number!");

    // Holy wtf syntax
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Based af rust loop keyword
    loop {
        println!("Please input your guess.");

        // This is a variable asignation, by default, variables are inmutable in Rust ?????WTF?????,
        // weird flex but ok, so we need the mut keyword be able to change it's value.
        // The right part of the asignation, we give it a value, in this case, we use the String type,
        // and the associated function new() (associated functions are called with ::) which returns a
        // new variable size empty string
        let mut guess = String::new();

        // This uses the stdin to get input from user
        io::stdin()
            // This line gets the input and appends its value to a mutable (mut) reference (&)
            .read_line(&mut guess)
            //This line is interesting, .read_line can return a enum (will learn more afterwards)
            //Result, which can be in a Ok or Err state (this states are called variants). .expect() crashes the program with the
            //message passed as an argument in case of Err
            .expect("Failed to read line");

        // This is variable shadowing, interesting but dangerous behavior. also, we type the intger as
        // an unsigned 32 bit number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // match is the way to assess a valid variant value, each "arm" (the possible outcomes) can
        // run code depending on the value
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
