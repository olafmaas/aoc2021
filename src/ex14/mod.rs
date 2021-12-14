use crate::generic::io::get_lines;
use crate::generic::parse::split_line;

use itertools::Itertools;
use std::collections::HashMap;

pub fn run() {
    let mut lines = get_lines("input/ex14/ex1");

    let template = lines.remove(0);
    lines.remove(0);

    let map: HashMap<(char, char), char> = lines
        .into_iter()
        .map(|line| split_line(line, " -> "))
        .map(|segments| {
            let mut chars = segments[0].chars();
            let first = chars.next().unwrap();
            let second = chars.next().unwrap();

            ((first, second), segments[1].chars().next().unwrap())
        })
        .collect();

    part1(template.clone(),  &map);
    part2(template, &map);
}

fn part1(mut template: String, map: &HashMap<(char, char), char>) {
    for i in 0..10 {
        println!("Step {}", i);
        let iter = template
            .chars()
            .tuple_windows()
            .map(|t| replace_chunk(t, &map));

        template = template.chars().interleave(iter).collect();
    }

    let counts = template.chars().counts();

    let min = counts.values().min().unwrap();
    let max = counts.values().max().unwrap();

    println!("{}", max - min)
}

fn part2(template: String, map: &HashMap<(char, char), char>) {
    let mut counts = template.chars().counts();
    let mut tuple_counts = template.chars().tuple_windows::<(char, char)>().counts();

    for i in 0..40 {
        println!("Step {}", i);

        let (tuples, chars): (Vec<Vec<((char, char), usize)>>, Vec<(char, usize)>) = tuple_counts
            .iter()
            .map(|(tuple, usize)| {
                let (c, new_tuple) = replace_chunk_smart(tuple, map);

                let v = new_tuple.into_iter().map(|t| (t, *usize)).collect();
                (v, (c, *usize))
            })
            .unzip();

        tuple_counts = HashMap::new();

        tuples
            .into_iter()
            .flat_map(|t| t)
            .for_each(|(t, i): ((char, char), usize)| { 
                tuple_counts.entry(t).and_modify(|v| *v += i).or_insert(i);
            });

        chars.into_iter().for_each(|(t, new_value)| {
            counts
                .entry(t)
                .and_modify(|value| *value += new_value)
                .or_insert(new_value);
        });

        println!("{:?}", counts);
        println!("{:?}", tuple_counts)
    }

    println!("{:?}", counts);

    let min = counts.values().min().unwrap();
    let max = counts.values().max().unwrap();

    println!("{}", max - min)
}

fn replace_chunk_smart(
    (c1, c3): &(char, char),
    map: &HashMap<(char, char), char>,
) -> (char, Vec<(char, char)>) {
    let c2 = map.get(&(*c1, *c3)).unwrap();

    (*c2, vec![(*c1, *c2), (*c2, *c3)])
}

fn replace_chunk((c1, c3): (char, char), map: &HashMap<(char, char), char>) -> char {
    let c2 = map.get(&(c1, c3)).unwrap();

    *c2
}
