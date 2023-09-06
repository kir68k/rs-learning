use std::{thread, time::{self, Instant}};
use rand::Rng;

pub fn return_value() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 100 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

// Example function for loops
// Prints a random number between
// a very large range every second
pub fn agony128() {
    // Create vars for time amounts
    let one_sec = time::Duration::from_secs(1);
    let ten_ms = time::Duration::from_millis(10);
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
        thread::sleep(ten_ms);
    };
}

pub fn loop_names() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 8 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count +=1;
    }
    println!("End count = {count}");
}

pub fn while_loop() {
    let mut number: i128 = rand::thread_rng().gen_range(i128::MIN..i128::MAX);

    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    println!("OJADSKLZJXLKCJKXCKJZJCNZL");
}

pub fn array_loop() {
    let awa: [i128; 6] = [i128::MIN, 10, 20, 40, 80, i128::MAX];

    for value in awa {
        println!("the value is: {value}");
    }
}
