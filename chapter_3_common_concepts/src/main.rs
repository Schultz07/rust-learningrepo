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
}
