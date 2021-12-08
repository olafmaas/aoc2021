use itertools::Itertools;
use std::collections::HashSet;

use crate::generic::io::get_lines;
use crate::generic::parse::as_int;
use crate::generic::parse::split_line;

type Input = (Vec<String>, Vec<String>);
type MarkedDisplay = (isize, HashSet<String>);

pub fn run() {
    let input: Vec<Input> = get_lines("input/ex8/ex1")
        .into_iter()
        .map(parse_line)
        .collect();

    part1(input.clone());
    part2(input);
}

fn parse_line(line: String) -> Input {
    let mut line = split_line(line, "|");

    let output = split_line(line.pop().unwrap().trim().to_owned(), " ");
    let observations = split_line(line.pop().unwrap().trim().to_owned(), " ");

    (observations, output)
}

fn part1(input: Vec<Input>) {
    let count = input
        .into_iter()
        .flat_map(|s| mark_all(s.1))
        .filter(|t| t.0 > -1)
        .count();

    println!("count {:?}", count);
}

fn mark_all(displays: Vec<String>) -> Vec<MarkedDisplay> {
    displays.into_iter().map(mark1478).collect()
}

fn mark1478(display: String) -> MarkedDisplay {
    let display_set = HashSet::from_iter(
        split_line(display, "")
            .into_iter()
            .filter(|a| !a.is_empty()),
    );

    match display_set.len() {
        2 => (1, display_set),
        3 => (7, display_set),
        4 => (4, display_set),
        7 => (8, display_set),
        _ => (-1, display_set),
    }
}

fn part2(input: Vec<Input>) {
    let sum: i64 = input.into_iter().map(solve).sum();

    println!("sum {:?}", sum);
}

fn solve((observations, output): Input) -> i64 {
    let marked_observed: Vec<MarkedDisplay> = observations.into_iter().map(mark1478).collect();
    let marked_output: Vec<MarkedDisplay> = output.into_iter().map(mark1478).collect();

    let result_string: String = marked_output
        .into_iter()
        .map(|d| mark6(d, &marked_observed))
        .map(|d| mark9(d, &marked_observed))
        .map(|d| mark0(d))
        .map(|d| mark3(d, &marked_observed))
        .map(|d| mark5(d, &marked_observed))
        .map(|d| mark2(d))
        .map(|(i, _): MarkedDisplay| i.to_string())
        .collect_vec()
        .join("");

    as_int(result_string.as_ref())
}

fn mark(
    (mark, display_set): MarkedDisplay,
    condition: &dyn Fn(&HashSet<String>) -> bool,
    on_true: isize,
) -> MarkedDisplay {
    if condition(&display_set) && mark == -1 {
        (on_true, display_set)
    } else {
        (mark, display_set)
    }
}

fn mark6(display: MarkedDisplay, observed: &Vec<MarkedDisplay>) -> MarkedDisplay {
    let on = find_display(&observed, 1);

    mark(
        display,
        &|set: &HashSet<String>| set.len() == 6 && on.intersection(&set).count() != 2,
        6,
    )
}

fn mark9(display: MarkedDisplay, observed: &Vec<MarkedDisplay>) -> MarkedDisplay {
    let on = find_display(&observed, 4);

    mark(
        display,
        &|set: &HashSet<String>| set.len() == 6 && on.intersection(&set).count() == 4,
        9,
    )
}

fn mark0(display: MarkedDisplay) -> MarkedDisplay {
    mark(display, &|set: &HashSet<String>| set.len() == 6, 0)
}

fn mark3(display: MarkedDisplay, observed: &Vec<MarkedDisplay>) -> MarkedDisplay {
    let on = find_display(&observed, 7);

    mark(
        display,
        &|set: &HashSet<String>| set.len() == 5 && on.intersection(&set).count() == 3,
        3,
    )
}

fn mark5(display: MarkedDisplay, observed: &Vec<MarkedDisplay>) -> MarkedDisplay {
    let on = find_display(&observed, 4);

    mark(
        display,
        &|set: &HashSet<String>| set.len() == 5 && on.intersection(&set).count() == 3,
        5,
    )
}

fn mark2(display: MarkedDisplay) -> MarkedDisplay {
    mark(display, &|set: &HashSet<String>| set.len() == 5, 2)
}

fn find_display<'a>(others: &'a Vec<MarkedDisplay>, num: isize) -> &'a HashSet<String> {
    &others.iter().find(|s| s.0 == num).unwrap().1
}
