use std::{thread, time};
use rand::Rng;

// Example function for conditions
fn cond() {
    // Set var condition to true
    let condition = true;
    // Create var number of type u128
    let number: u128 =
        // If var condition is true, execute first block
        // If not, execute the latter block, after else
        if condition {
            34298532453453452345325
        } else {
            3924892384234234234289435
        };
    // Print the value of var number
    println!("the value of number is {number}");
}

// Example function for loops
// Prints a random number between
// a very large range every second
fn agony128() {
    // Create var one_sec, which is 1 second of time
    let one_sec = time::Duration::from_secs(1);
    // Create a loop, main part of the function
    loop {
        // Create var rand_number of type i128
        // Value is a random number between the
        // minimum and maximum amount in i128
        let rand_number: i128 = rand::thread_rng().gen_range(i128::MIN..i128::MAX);
        // Print the value of rand_number
        println!("Random number: {rand_number}");
        // Sleep (don't do anything) for one second
        // After that, repeat the loop with a new number
        thread::sleep(one_sec);
    };
}

// Main function
fn main() {
    let number: i128 = 95329843958439;

    // Print true if var number is less than this amount
    // Else, print false
    if number < 23895462598384234 {
        println!("true");
    } else {
        println!("false");
    }

    // If var number is more than this, run cond()
    // Else, print "hi"
    if number > 328993284234 {
        cond();
    } else {
        println!("hi");
    };

    // Print a random number every second
    agony128();
}
