use std::cell::LazyCell;

use regex::Regex;

const REGEX: LazyCell<Regex> = LazyCell::new(|| Regex::new(r"\bmul\((\d+),(\d+)\)").unwrap());

enum Instructions {
    Mul(i32, i32),
}

impl Instructions {
    fn from_str(s: &str) -> Vec<Self> {
        REGEX
            .captures_iter(s)
            .map(|captures| {
                let a = captures[1].parse().unwrap();
                let b = captures[2].parse().unwrap();
                Self::Mul(a, b)
            })
            .collect()
    }

    fn eval(&self) -> i32 {
        match self {
            Self::Mul(a, b) => a * b,
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let instructions: Vec<Instructions> = Instructions::from_str(input);

    let products = instructions
        .iter()
        .map(|instruction| instruction.eval())
        .collect::<Vec<i32>>();

    let sum = products.iter().sum::<i32>();

    println!("Sum of all products: {}", sum);
}
