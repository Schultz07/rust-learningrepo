use std::io;

fn main() {
    loop {
        println!("Insert the nth fibonacci number you want to find:");
        let mut nth_number = String::new();
        io::stdin()
            .read_line(&mut nth_number)
            .expect("Failed to read input");
        let nth_number: usize = match nth_number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Insert a valid number!");
                continue;
            }
        };
        let fibonacci: u128 = nth_fibonacci(nth_number);
        println!("{fibonacci}");
        return;
    }
}

fn nth_fibonacci(n: usize) -> u128 {
    if n <= 1 {
        return n as u128;
    }

    return nth_fibonacci(n - 1) + nth_fibonacci(n - 2);
}
