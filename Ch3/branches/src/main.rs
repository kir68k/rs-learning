mod loop_funcs;

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

// Main function
fn main() {
    //let number: i128 = 95329843958439;

    // Print true if var number is less than this amount
    // Else, print false
    //if number < 23895462598384234 {
    //    println!("true");
    //} else {
    //    println!("false");
    //}

    // If var number is more than this, run cond()
    // Else, print "hi"
    //if number > 328993284234 {
    //    cond();
    //} else {
    //    println!("hi");
    //};

    // Print a random number every ten ms
    //loop_funcs::agony128();

    loop_funcs::return_value();

    //loop_funcs::loop_names();
    //loop_funcs::while_loop();

    loop_funcs::array_loop();
}
