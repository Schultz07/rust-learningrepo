fn main() {
    let mut s1 = String::from("hello");

    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}.");

    change(&mut s1);
    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}.");

    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{r1} and {r2}");

    let r3 = &mut s;
    println!("{r3}");
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
