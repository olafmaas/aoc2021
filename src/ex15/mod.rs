use itertools::enumerate;
use itertools::izip;
use petgraph::algo::dijkstra;
use petgraph::graph::NodeIndex;

use crate::generic::io::get_lines;
use crate::generic::parse::as_int;
use crate::generic::parse::split_line;

use itertools::Itertools;
use std::collections::HashMap;

use petgraph::Graph;
use petgraph::algo::astar;


pub fn run() {
    let iter = get_lines("input/ex15/ex1")
        .into_iter()
        .enumerate()
        .flat_map(|(y, line)| {
            split_line(line, "")
                .into_iter()
                .filter(|x| !x.is_empty())
                .enumerate()
                .map(|(x, char)| ((x as isize, y as isize), as_int(&char)))
                .collect_vec()
        });

    let map: HashMap<(isize, isize), i64> = HashMap::from_iter(iter);

    // println!("{:?}", map);
    // part1(map.clone());
    part1(&map);

    let new_map = map_extend(map);

    part1(&new_map)
}

fn map_extend(map: HashMap<(isize, isize), i64>) -> HashMap<(isize, isize), i64>  {
  let width = map.keys().map(|(x, _)| x).max().unwrap() + 1;
  let height = map.keys().map(|(_, y)| y).max().unwrap() + 1;

  let mut result = map.clone();
  
  for dy in 0..5 {
    for dx in 0..5 {
      if dx == 0 && dy == 0 {
        continue
      }

      result.extend(
        map
        .iter()
        .map(|((x, y), w)| ((x + (width*dx), y + (dy*height) ), val(dx as i64, dy as i64, *w)))
      );

    }
  }

  // println!("{:?}", result);
  
  result
}

fn val(dx: i64, dy: i64, w: i64) -> i64 {
  let value = w + dx + dy;

  if value > 9 {
    value - 9 
  } else {
    value
  }
}

fn part1(map: &HashMap<(isize, isize), i64>) {
    let mut graph = Graph::<(isize, isize), i64>::new();

    let index_map : HashMap<(isize, isize), (NodeIndex, i64)> = HashMap::from_iter(map.iter().map(|((x, y), w)| {
        ((*x,*y), (graph.add_node((*x, *y)), *w))
    }));

    index_map.iter().for_each(|((x, y), (index, weight))| {
      neighbours(x, y)
      .into_iter()
      .filter(|t| index_map.contains_key(t))
      .for_each(|neighbour| {
        let (i, _) = index_map.get(&neighbour).unwrap();
        graph.add_edge(*i, *index, *weight);
      })
    });

    let max_x = index_map.keys().map(|(x, _)| x).max().unwrap();
    let max_y = index_map.keys().map(|(_, y)| y).max().unwrap();
    let (end_pos, _) = index_map.get(&(*max_x, *max_y)).unwrap();
    let (start_pos, _) = index_map.get(&(0, 0)).unwrap();

    // println!("{:?}", (max_x, max_y));

    // let map  = dijkstra(&graph, *start_pos, Some(*end_pos), |w| *(w.weight()));
    // let map  = dijkstra(&graph, *start_pos, Some(*end_pos), |w| *(w.weight()));

    let (sum, _) = astar(&graph, *start_pos, |f| f == *end_pos, |e| *e.weight(), |_| 0).unwrap();

    // let sum : i64 = *map.get(end_pos).unwrap();

    println!("Sum {}", sum)
}

fn neighbours(x: &isize, y: &isize) -> Vec<(isize, isize)> {
    vec![(x - 1, *y), (x + 1, *y), (*x, y - 1), (*x, y + 1)]
}

