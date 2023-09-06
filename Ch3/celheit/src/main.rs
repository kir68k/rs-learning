use std::io;

fn convert() {
    println!("Please enter a temperature in Fahrenheit");

    let mut fahrenheit = String::new();

    io::stdin()
        .read_line(&mut fahrenheit)
        .expect("Failed to read line");

    let fahrenheit: f64 = match fahrenheit.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Expected a number"),
    };

    let celsius: f64 =
        (fahrenheit - 32.0) * 5.0 / 9.0;

    let kelvin: f64 =
        (fahrenheit - 32.0) * 5.0 / 9.0 + 273.15;

    let rankine: f64 =
        fahrenheit + 459.67;

    println!("");
    println!("{fahrenheit}°F is:");
    println!("-- {celsius}°C");
    println!("-- {kelvin}K");
    println!("-- {rankine}°R");
}

fn main() {
    convert();
}
