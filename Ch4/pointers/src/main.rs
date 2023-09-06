// Create a function, takes a mutable string ref as an argument
// Taken from: https://micouy.github.io/rust-dereferencing/
fn modify(arg: &mut String) {
    // Append the following character(s) to argument
    arg.push_str("z");
}

// Create a function, takes a string reference as an argument
// returns a string reference
fn first_word(s: &str) -> &str {
    // Create an immutable variable, returns a byte slice
    // from the contents of `s`
    let bytes = s.as_bytes();

    // Create an iterator, where `i` is the last part of the slice
    // and `&item` is a reference to `item`
    for (i, &item) in bytes.iter().enumerate() {
        // If `item` is a space character, return string with slice
        // starting from the beginning, ending at value before the space
        if item == b' ' {
            return &s[0..i];
        }
    }

    // Reference to `s` with the full range
    // Required for our iterator, as it always requires a return
    &s[..]
}

fn main() {
    // Create a mutable variable, string
    let mut latin = String::from("abcdefghijklmnopqrstuvwxy");

    // Create an immutable variable, slice that points
    // to the first 10 values of the alphabet's index
    let la_first_ten = &latin[0..10];

    // Create an immutable variable, slice that points
    // to the last 10 values in the alphabet's index
    let la_last_ten = &latin[16..25];

    // Print first ten and last ten letters of
    // the current alphabet, without `z`
    println!("{la_first_ten}");
    println!("{la_last_ten}");

    // Push z at the end of the alphabet
    modify(&mut latin);

    // Print the modified alphabet
    println!("{latin}");

    // Create a mutable variable, string
    let mut s = String::from("hewwo world");
    // Create an immutable variable, calling `first_word`
    let word = first_word(&s[0..6]);

    let string_literal = "hewwo world";

    // Because string literals are already slices, the slice
    // syntax is not required
    let literal_word = first_word(&string_literal);

    // Print immutable variable `word`
    println!("the first word is: {}", word);
    println!("the first word of string_literalis: {}", literal_word);

    // Clear contents of the mutable variable `s`
    s.clear();

    // This works because we're calling `s`, which
    // is a mutable string. If we called `word` it would not compile
    // as `word` is immutable.
    println!("after clear: {}", s);
}
