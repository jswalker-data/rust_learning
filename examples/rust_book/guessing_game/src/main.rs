// Pull in input/output library from the standard library
// Standard library brings in prelude use a 'use' to bring in anything else
use std::io;


// Rng is a trait that defines methods that rand num gens implement
use rand::Rng;

// Standard main function entry point
fn main() {
    println!("Guess the number!");

    // Generator used is local to current thread and seeded by os
    // Call gen_range defined by Rng trait
    // Range 1..=100 is inclusive on lower and upper bounds
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");


    // Let creates a variable, mutable variable so we can change it
    // Guess becomes a new instance of a string, growable UTF-8 encoded
    // ::new() is associated function of the String type, makes a new empty string
    // This line creates a mutable variable currently bound to a new, empty string
    let mut guess = String::new();
    

    // Type represents a handle to standard input of a terminal
    // Next call read_line method on the standard input handle and where to store it
    // & is a reference to the mutable variable guess, refs are immutable by default so we make it mutable
    // read_line has a 'Result' with variant err and ok. err will trigger expect
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
