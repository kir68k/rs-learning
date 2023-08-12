use std::{thread, time};
use rand::Rng;

fn cond() {
    let condition = true;
    let number: u128 =
        if condition {
            34298532453453452345325
        } else {
            3924892384234234234289435
        };
    println!("the value of number is {number}");
}

fn agony128() {
    let one_sec = time::Duration::from_secs(1);
    loop {
        let rand_number: i128 = rand::thread_rng().gen_range(i128::MIN..i128::MAX);
        println!("Random number: {rand_number}");
        thread::sleep(one_sec);
    };
}

fn main() {
    let number: i128 = 95329843958439;

    if number < 23895462598384234 {
        println!("true");
    } else {
        println!("false");
    }

    if number > 328993284234 {
        cond();
    } else {
        println!("hi");
    };

    agony128();
}
