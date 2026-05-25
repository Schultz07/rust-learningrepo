use std::io;

fn main() {
    println!("Enter thy string, person:");
    let mut entered_value = String::new();

    io::stdin()
        .read_line(&mut entered_value)
        .expect("Failed to read line");
    let vogals = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];

    let split_string = entered_value.split_whitespace();

    let mut result: String = String::new();

    for i in split_string {
        let mut v: Vec<char> = i.chars().collect();

        if vogals.contains(&v[0]) {
            let mut hay_values = vec!['-', 'h', 'a', 'y'];
            v.append(&mut hay_values);
        } else {
            let x = v.remove(0);
            let mut hay_values = vec!['-', x, 'h', 'a', 'y'];
            v.append(&mut hay_values);
        }
        let formatted_string: String = v.into_iter().collect();
        result.push_str(&formatted_string);
        result.push(' ');
    }
    let result = result.trim();
    println!("{result}");
}
