use itertools::enumerate;

use crate::generic::io::get_lines;
use crate::generic::parse::as_int;
use crate::generic::parse::split_line;

use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::Add;
use itertools::Itertools;

pub fn run() {
  let iter = 
    get_lines("input/ex9/ex1")
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

  let map : HashMap<(isize, isize), i64> = HashMap::from_iter(iter);

  println!("{:?}", map);
  part1(map.clone());
  part2(map.clone());

}

fn part1(map: HashMap<(isize, isize), i64>) {

  let sum : i64 = map
  .iter()
  .filter(|((x,y), val)| {
    adjacent(&map, *x, *y)
    .into_iter()
    .filter(|neighbour| neighbour <= val)
    .count()
    == 0
  })
  .map(|t| t.1 + 1)
  .sum();

  println!("Sum of risk= {:?}", sum)

}

fn adjacent(map: &HashMap<(isize, isize), i64>, x: isize, y: isize) -> Vec<i64> {
  vec![
    map.get(&(x-1, y)).map(|i| *i).unwrap_or(i64::MAX),
    map.get(&(x+1, y)).map(|i| *i).unwrap_or(i64::MAX),
    map.get(&(x, y+1)).map(|i| *i).unwrap_or(i64::MAX),
    map.get(&(x, y-1)).map(|i| *i).unwrap_or(i64::MAX)
  ]
}


fn part2(map: HashMap<(isize, isize), i64>) {
  let low_points : Vec<(isize, isize)> = map
  .iter()
  .filter(|((x,y), val)| {
    adjacent(&map, *x, *y)
    .into_iter()
    .filter(|neighbour| neighbour <= val)
    .count()
    == 0
  })
  .map(|((x, y), _)| (*x, *y))
  .collect_vec();

  let biggest_bassins = low_points
  .into_iter()
  .map(|x| as_bassin(x, &map))
  .sorted_by(|seta, setb| setb.len().cmp(&seta.len()))
  .take(3)
  .collect_vec();

  println!("top 3 {:?}", biggest_bassins);

  println!("Mul: {:?}", biggest_bassins[0].len() * biggest_bassins[1].len() * biggest_bassins[2].len());
}



fn as_bassin((x,y): (isize, isize), map: &HashMap<(isize, isize), i64>) -> HashSet<(isize, isize)> {
  let mut set = HashSet::new();

  find_bassin_pos((x, y), map, &mut set);

  set
}

fn find_bassin_pos((x,y): (isize, isize), map: &HashMap<(isize, isize), i64>, set: &mut HashSet<(isize, isize)>) {
  if map.get(&(x,y)).map(|i| *i).unwrap_or(9) == 9 {
    return;
  }

  set.insert((x,y));

  let clone = set.clone();

  adjacent_pos(x, y)
  .into_iter()
  .filter(|pos| !clone.contains(pos))
  .for_each(|pos| find_bassin_pos(pos, map, set))
}

fn adjacent_pos(x: isize, y: isize) -> Vec<(isize, isize)> {
  vec![
    (x+1, y),
    (x-1, y),
    (x, y+1),
    (x, y-1)
  ]
}