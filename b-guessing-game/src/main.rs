// Using more than an import from a source
use std::{cmp::Ordering, io};
// Using dependency
use rand::Rng;

fn main() {
    println!("Guess the number");
    print!("Please input your guess: ");

    // secret_number is immutable (const)
    // thread_rng is the number generator
    // gen_range a function of the number generator to really generate using the range-expression as argument
    // The equals sign (=) at range-expression assure that 100 is included in return
    // Auto infer that is a number (i32 by default)
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // loop create an infinite loop
    loop {
        // let means to create the variable
        // mut means the variable can change (variables are immutable by default)
        // :: indicate that the function new() is associated of String type
        // Auto Infer that is a String
        // Not using this to compare, just reading because every reading is a string
        let mut guess = String::new();
        io::stdin()
            // This reads an entry from user and insert into the variable
            // Like variables, references are immutable by default, that's why use mut
            // It returns a Result type (Ok or Err)
            .read_line(&mut guess)
            // Result has expect() method called when Err is returned, causing the program to crash and presenting the arguments
            // If Result is Ok, only re-returns Ok
            // Compiler is waiting expect function when returning Result (or will warn you so)
            .expect("Failed to read line");

        // Has the same variable name as string version, but the string version is shadowed by this, so the new version will be used from now on inside this scope
        // This value is immutable and it will be re-declared every loop
        // trim() eliminates white spaces (including \n or \r\n from user entry)
        // parse() will try to parse the value to the type of the assigned variable
        let guess: u32 = match guess.trim().parse() {
            // Using "match" in the beggining, we can use this block to deal with Result specifically
            // If Ok, returns the number to the variable
            Ok(num) => num,
            // If Err, runs the next loop iterations, ignoring the rest of the code in this iteration
            // The underscore (_) means anything
            Err(_) => continue,
        };

        // cmp() can compare anything, as long they are the same type, and returns Ordering value (Less, Greater or Equal)
        // match is used to directly match a value (switch-case like) in a short structure
        // match guarantees that all the cases (patterns) should be covered
        // Rust has a string, static type system, but it also has type inference
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                // breaks the look
                break;
            }
        }
    }

    println!("Game Over");
}
