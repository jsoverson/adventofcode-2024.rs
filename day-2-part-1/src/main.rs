const MIN_THRESHOLD: i32 = 1;
const MAX_THRESHOLD: i32 = 3;

fn main() {
    let input = include_str!("../input.txt");

    let reports: Vec<Vec<i32>> = input
        // split input by newline
        .lines()
        // map each line to a list of numbers by splitting on
        // spaces and parsing each substring as a number
        .map(|line| {
            line.split(' ')
                .map(|x| x.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        // compare every adjacent pair of numbers and ensure
        // that the difference between them is within the threshold
        .filter(|levels| {
            levels.windows(2).all(|window| {
                let diff = (window[0] - window[1]).abs();
                diff >= MIN_THRESHOLD && diff <= MAX_THRESHOLD
            })
        })
        // ensure all levels in each report are either increasing or decreasing
        .filter(|levels| levels.is_sorted() || levels.iter().rev().collect::<Vec<_>>().is_sorted())
        .collect();

    println!("Num safe: {}", reports.len());
}
