fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}")
    }

    println!("The value of x is: {x}");
    let spaces = "   ";
    let spaces = spaces.len();
    println!("{spaces}");

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{guess}");

    let sum = 5 + 10;

    let difference = 95.5 - 4.3;

    let product = 4 * 30;

    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;

    let remainder = 43 % 5;
    println!("{sum}, {difference}, {product}, {quotient}, {truncated}, {remainder}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let y = tup.1;
    println!("The value of y is: {y}");

    let a: [u8; 5] = [1, 2, 3, 4, 5];
    let element = a[0];
    println!("{element}");
    another_function(5, 'f');

    let number = 9;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");
    loop {
        println!("again!");
        break;
    }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
    manual_while_loop();

    let mut number: u8 = 3;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is (while): {}", a[index]);

        index += 1;
    }

    for element in a {
        println!("the value is (for): {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn manual_while_loop() {
    let mut counter: u8 = 0;
    loop {
        counter += 1;
        if counter == 10 {
            break;
        } else {
            println!("Counting {counter}");
        }
    }
    println!("Counter is {counter}");
}

fn five() -> u8 {
    5
}

fn another_function(x: u8, unit_label: char) {
    println!("Another function! {x} {unit_label}");

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
    let x = five();
    println!("The value of x is: {x}");

    let z = plus_one(5);
    println!("The value of z is: {z}");
}

fn plus_one(x: u8) -> u8 {
    // hello world
    // This is a longer comment bla bla bla
    // lol
    x + 1 // I can put them here too
}
