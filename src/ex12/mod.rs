use itertools::Itertools;

use crate::generic::io::get_lines;
use crate::generic::parse::split_line;

use std::{
    collections::{HashMap, HashSet},
    path,
};

type Graph = HashMap<String, Vec<String>>;

pub fn run() {
    let input = parse_graph(get_lines("input/ex12/ex1"));

    part1(&input);
    part2(&input)
}

fn parse_graph(lines: Vec<String>) -> HashMap<String, Vec<String>> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    lines
        .into_iter()
        .map(|line| split_line(line, "-"))
        .for_each(|mut nodes| {
            let right = nodes.pop().unwrap();
            let left = nodes.pop().unwrap();

            graph
                .entry(left.clone())
                .and_modify(|neighbours| neighbours.push(right.clone()))
                .or_insert(vec![right.clone()]);
            graph
                .entry(right)
                .and_modify(|neighbours| neighbours.push(left.clone()))
                .or_insert(vec![left]);
        });

    graph
}

fn part1(graph: &Graph) {
    let paths = search(graph, &("start".to_owned()), HashSet::new(), false);

    println!("sum {}", paths.len())
}

fn search(
    graph: &Graph,
    node: &String,
    mut visited: HashSet<String>,
    extra_visit: bool,
) -> HashSet<Vec<String>> {
    // println!("node: {}", node);
    if node == "end" {
        return HashSet::from_iter(vec![vec![node.clone()]]);
    }

    let mut paths = HashSet::new();

    if node.to_uppercase() != *node {
        let original_visited = visited.clone();

        visited.insert(node.clone());

        if extra_visit && node != "start" {
            graph
                .get(node)
                .unwrap()
                .iter()
                .filter(|node| !visited.contains(*node))
                .for_each(|node| {
                    let found_paths = search(graph, node, original_visited.clone(), false);

                    paths.extend(found_paths.into_iter())
                });
        }
    }

    let sum = graph
        .get(node)
        .unwrap()
        .iter()
        .filter(|node| !visited.contains(*node))
        .for_each(|node| {
            let found_paths = search(graph, node, visited.clone(), extra_visit);

            paths.extend(found_paths.into_iter())
        });

    HashSet::from_iter(paths.into_iter().map(|mut path| {
      path.push(node.clone());

      path
    }))
}

fn part2(graph: &Graph) {
    let sum = search(graph, &("start".to_owned()), HashSet::new(), true);

    println!("paths {:?}", sum);
    println!("sum with exta {}", sum.len())
}
