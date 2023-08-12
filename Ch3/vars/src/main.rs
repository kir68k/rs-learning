use std::io;

fn argument_function(x: u128) {
    println!("You have entered: {x}");
}

fn interactive_argument() {
    loop {
        println!("Please enter a number.");

        let mut number = String::new();

        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line");

        let number: u128 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        argument_function(number);
        break;
    }
}

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

fn ttest() {
    let x: (i128, f32, u8) = (5000000325983483859389243, 6.2389413, 255);

    let float = x.1;

    println!("testjlkcjlkdsglk {float}");
}

fn array_test() {
    let x: [i64; 4] = [1, 2, 9, 12309123];

    let a = x[0];

    println!("{a}");
}

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
