use std::io;

// Print the value of argument x
fn argument_function(x: u128) {
    println!("You have entered: {x}");
}

// Example for an interactive CLI program
fn interactive_argument() {
    // Start a loop
    loop {
        println!("Please enter a number.");

        // Create a mutable var number, which is
        // a user-defined string
        let mut number = String::new();

        // Read from standard input
        // Set var number to whatever was typed
        // Error if unable to read from stdin, shouldn't happen
        // on a healthy OS
        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line");

        // Set number to type u128
        // If number is actually a number type, succeed
        // If it's not, repeat loop
        let number: u128 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Call function with value of number
        // and break the loop
        argument_function(number);
        break;
    }
}

// A function which compiles, but errors
// if you enter an index that doesn't exist
fn invalid_array() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}

fn hi() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("{c} {z} {heart_eyed_cat}");
}

// Tuple test function
fn ttest() {
    let x: (i128, f32, u8) = (5000000325983483859389243, 6.2389413, 255);

    let float = x.1;

    println!("testjlkcjlkdsglk {float}");
}

// Aray test function
fn array_test() {
    let x: [i64; 4] = [1, 2, 9, 12309123];

    let a = x[0];

    println!("{a}");
}

// Main function
fn main() {
    let x = 10;
    
    let x = x + 1;

    {
        let x = x * 5;
        println!("x is here {x}");
    }

    println!("x is here {x}");

    let sum = 5 + 10;

    let delta = 4 - 1;

    let prod: i128 = 4 * 1701411834604692317316873037158841;

    let quotient = 56.7 / 32.2;

    let trunc = -10 / 5;

    let remainder = 44 % 5;

    println!("hi\n{sum}\n{delta}\n{prod}\n\n{quotient}\n{trunc}\n\n{remainder}");

    // hi();

    // ttest();

    // array_test();

    // invalid_array();

    // argument_function(1701411834604692317316873037158841);
    interactive_argument();
}
