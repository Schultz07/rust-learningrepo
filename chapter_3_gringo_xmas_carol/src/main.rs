fn main() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let gifts = [
        "A partridge in a pear tree",
        "Two turtle doves and",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];
    let mut counter: usize = 0;
    for day in days {
        counter += 1;
        println!("On the {day} day of Christmas, my true love sent to me");
        for gift_index in (0..counter).rev() {
            let gift = gifts[gift_index];
            println!("{gift}");
        }
        println!();
    }
}
