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
        .filter(is_valid_dampener)
        .collect();

    println!("Num safe: {}", reports.len());
}

fn is_valid_dampener(report: &Vec<i32>) -> bool {
    is_valid(report, &[])
        || (0..(report.len())).any(|i| is_valid(&report[0..i], &report[(i + 1)..]))
}

fn is_valid(head: &[i32], tail: &[i32]) -> bool {
    let combined = [head, tail].concat();
    combined.windows(2).all(|window| {
        let diff = (window[0] - window[1]).abs();
        diff >= MIN_THRESHOLD && diff <= MAX_THRESHOLD
    }) && (combined.is_sorted() || combined.iter().rev().collect::<Vec<_>>().is_sorted())
}
