use std::io;

fn main() {
    println!("1. Celsius to Farenheit");
    println!("2. Farenheit to Celsius");
    let mut unit_type = String::new();
    io::stdin()
        .read_line(&mut unit_type)
        .expect("Failed to read input!");
    let unit_type: u8 = match unit_type.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            invalid_number();
            return;
        }
    };
    match unit_type {
        1 => convert_celsius_farenheit(),
        2 => convert_farenheit_celsius(),
        _ => {
            println!("Please, insert a number within the range of options.");
            panic!();
        }
    };
}

fn convert_farenheit_celsius() {
    println!("Insert temperature: ");
    let mut temperature = String::new();
    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read input!");
    let temperature: f32 = match temperature.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            invalid_number();
            panic!();
        }
    };
    let temperature = (temperature - 32.0) * (5.0 / 9.0);
    println!("{temperature} ºC");
}

fn convert_celsius_farenheit() {
    println!("Insert temperature: ");
    let mut temperature = String::new();
    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read input!");
    let temperature: f32 = match temperature.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            invalid_number();
            panic!();
        }
    };
    let temperature = (temperature * (9.0 / 5.0)) + 32.0;
    println!("{temperature} ºF");
}

fn invalid_number() {
    println!("Please, insert a valid number.");
}
