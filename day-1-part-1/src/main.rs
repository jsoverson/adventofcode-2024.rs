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
    let sum: u32 = col1
        .iter()
        .zip(col2.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum();
    println!("{}", sum);
}
