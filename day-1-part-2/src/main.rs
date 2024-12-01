use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    let (mut col1, mut col2): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|line| {
            let (l, r) = line.split_once("   ").unwrap();
            (l.parse::<i32>().unwrap(), r.parse::<i32>().unwrap())
        })
        .unzip();

    col1.sort();
    col2.sort();
    let frequency: HashMap<i32, i32> = col2.iter().fold(HashMap::new(), |mut acc, &x| {
        *acc.entry(x).or_insert(0) += 1;
        acc
    });

    let sum: i32 = col1
        .iter()
        .map(|x| x * frequency.get(x).copied().unwrap_or_default())
        .sum();

    println!("{}", sum);
}
