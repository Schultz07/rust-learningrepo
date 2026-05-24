enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    if let Some(third) = v.get(2) {
        println!("The third element is {third}");
    } else {
        println!("There is no third element!");
    }
    for i in &v {
        println!("{i}");
    }

    for i in &mut v {
        *i += 50;
        println!("{i}");
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("Blue")),
    ];

    let mut s = String::new();
    let data = "initial contents";

    let s = data.to_string();

    let s = "Testing contents".to_string();

    let mut s = String::from("initial contents");
    s.push_str(" more data");
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let s3 = s1 + &s2;

    let s1 = String::from("Tic");
    let s2 = String::from("Tac");
    let s3 = String::from("Toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");

    for c in "Здá".chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }
}
