// Import our functions and structs
mod users;

fn main() {
    // Create mutable variable of an "Ab"user type
    // Defined in users.rs
    let mut sussy = users::Abuser {
        active: true,
        username: String::from("susser"),
        location: String::from("Islamabad, England"),
        sign_in_count: 131615211488133742069,
    };

    if sussy.active {
        println!("Old location of {}: {}", sussy.username, sussy.location);

        sussy.location = String::from("Beijing, West Taiwan");

        println!("New location of {}: {}", sussy.username, sussy.location);
        if sussy.sign_in_count < u128::MAX {
            println!("{} did not reach the max sign-in limit, yet...", sussy.username);
            println!("Current sign-in count of {}: {}", sussy.username, sussy.sign_in_count);
        }
    }
    
    // Create new immutable Abuesr named amonga
    let amonga = users::Abuser {
        // Change username from the one used in "sussy"
        username: String::from("amonga"),
        // Take the sign-in count from sussy, set u64::MAX to u128, subtract that value from
        // sussy.sign_in_count, and make that the value. We are not changing the actual value of
        // sussy's sign-in count. If `as u128` wasn't used, this would not be possible.
        sign_in_count: sussy.sign_in_count - u64::MAX as u128,
        // Set the rest of variables to the values of sussy.
        ..sussy
    };

    println!("The sign-in count of {} is: {}", amonga.username, amonga.sign_in_count);

    // Create another user with the build_abuser function
    let user2 = users::build_abuser("Belgrade, East Korea".to_string(), "nullptr".to_string());

    println!("Location of {} is: {}", user2.username, user2.location);
}
