use crate::generic::io::get_lines;
use crate::generic::parse::as_int;
use itertools::Itertools;

pub fn run() {
    let parsed_lines: Vec<i64> = get_lines("input/ex1/ex1")
        .into_iter()
        .map(|s| as_int(&s))
        .collect();

    part1(parsed_lines.clone());
    part2(parsed_lines);
}

fn part1(input: Vec<i64>) {
    let nr_of_increases = input
        .into_iter()
        .tuple_windows()
        .filter(|(a, b)| a < b)
        .count();

    println!("{:?}", nr_of_increases)
}

fn part2(input: Vec<i64>) {
    let nr_of_increases = input
        .into_iter()
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .filter(|(a, b)| a < b)
        .count();

    println!("{:?}", nr_of_increases)
}
