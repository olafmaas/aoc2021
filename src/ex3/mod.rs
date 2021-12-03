use crate::generic::io::get_lines;
use itertools::Itertools;
use std::iter;

pub fn run() {
    let parsed_lines: Vec<Vec<String>> = get_lines("input/ex3/ex1")
        .into_iter()
        .map(|s| s.chars().map(|s| s.to_string()).collect_vec())
        .collect();

    println!("{:?}", parsed_lines);
    part1(parsed_lines.clone());
    part2(parsed_lines);
}

fn part1(mut input: Vec<Vec<String>>) {
    let init: Vec<Vec<String>> = input
        .pop()
        .unwrap()
        .iter()
        .map(|s| vec![s.clone()])
        .collect();

    let res = input
        .into_iter()
        .fold(init, |mut acc, binary| {
            acc.iter_mut()
                .zip(binary)
                .for_each(|(vec, s)| {
                    vec.push(s.clone());
                });

            acc
        })
        .iter()
        .map(decide_longest)
        .join("");

    let gamma = usize::from_str_radix(res.as_str(), 2).unwrap();

    //Binary operators are for people who have documentation ;) 
    let max_binary = res.chars().map(|_| "1").join("");

    let epsilon = usize::from_str_radix(max_binary.as_str(), 2).unwrap() - gamma;

    println!("{:?}, {:?}", max_binary, res);
    println!("{:?}", gamma * epsilon)
}

fn part2(input: Vec<Vec<String>>) {
    let input = input;

    let oxygen_binary = find_number(input.clone(), 0, &|ones, zero| {
        if ones >= zero {
            "1"
        } else {
            "0"
        }
    });
    let co2_binary = find_number(input, 0, &|ones, zero| if ones < zero { "1" } else { "0" });

    println!("{:?}", oxygen_binary);
    println!("{:?}", co2_binary);

    println!(
        "{:?}",
        int_from_bin(oxygen_binary) * int_from_bin(co2_binary)
    )
}

fn find_number(
    input: Vec<Vec<String>>,
    pos: usize,
    determine_filter: &dyn Fn(usize, usize) -> &'static str,
) -> String {
    let len = input.len();

    let ones = input.iter().filter(|v| v[pos].as_str() == "1").count();

    let pattern = determine_filter(ones, len - ones);

    let filtered: Vec<Vec<String>> = input.into_iter().filter(|v| v[pos] == pattern).collect();

    if filtered.len() == 1 {
        filtered.first().unwrap().join("")
    } else {
        find_number(filtered, pos + 1, determine_filter)
    }
}

fn decide_longest(column: &Vec<String>) -> &str {
    let len = column.len();
    let ones = column.iter().filter(|c| *c == "1").count();

    if len - ones < ones {
        "1"
    } else {
        "0"
    }
}

fn int_from_bin(s: String) -> usize {
    usize::from_str_radix(&s, 2).unwrap()
}
