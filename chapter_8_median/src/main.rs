use std::collections::HashMap;

fn main() {
    let mut integers: Vec<i32> = vec![
        120, 9232, 8123, 213, 232, 231, 123, 3231, 122, 42, 32, 42, 645, 434, 123, 323, 1212, 239,
        898, 242, 423,
    ];

    integers.sort();

    let median: f32 = {
        if (integers.len() % 2) == 0 {
            let m1 = (integers.len() / 2) - 1;
            (integers[m1] + integers[m1 + 1]) as f32 / 2.0
        } else {
            let m = integers.len() / 2;
            integers[m] as f32
        }
    };

    println!("{median:?}");

    let mut integers_map: HashMap<i32, u32> = HashMap::new();

    for i in integers.iter() {
        let entry = integers_map.entry(*i).or_insert(0);
        *entry += 1;
    }

    let max_size = integers_map.iter().max_by(|x, y| x.1.cmp(y.1)).unwrap().1;

    let mut modes: Vec<i32> = integers_map
        .iter()
        .filter(|x| x.1 == max_size)
        .map(|x| *x.0)
        .collect();

    modes.sort();

    println!("{modes:?}");
}
