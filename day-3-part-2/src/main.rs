mod program;

use tracing::level_filters::LevelFilter;

use program::Program;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::TRACE)
        .init();

    let input = include_str!("../input.txt");

    let mut progam = Program::parse(input).unwrap();

    let result = progam.eval().unwrap();

    println!("Result: {}", result);
}
