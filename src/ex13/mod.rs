use crate::generic::io::get_lines;
use crate::generic::parse::as_int;
use crate::generic::parse::split_line;

use itertools::Itertools;
use std::collections::HashSet;

pub fn run() {
    let (coordinates, folds) = parse_input(get_lines("input/ex13/ex1"));

    println!("{:?}", coordinates);
    println!("{:?}", folds);

    part1(coordinates.clone(), folds.clone());
    part2(coordinates, folds)
}

fn part1(coordinates: Vec<(i64, i64)>, folds: Vec<(i64, i64)>) {

  let init_set = HashSet::from_iter(coordinates.into_iter());

  print_set(init_set.clone());

  let set = fold(init_set, &folds[0]);

  print_set(set.clone());
  println!("Points visible {}", set.len())
  
}

fn part2(coordinates: Vec<(i64, i64)>, folds: Vec<(i64, i64)>) {

  let set = folds
  .into_iter()
  .fold(HashSet::from_iter(coordinates.into_iter()), |acc, fold_ins| fold(acc, &fold_ins));

  

  print_set(set)
  
}


fn print_set(set: HashSet<(i64, i64)>) {
  println!("{:?}", set);

  let max_x = set.iter().map(|t| t.0).max().unwrap();
  let max_y = set.iter().map(|t| t.1).max().unwrap();

  for y in 0..max_y+1 {
    for x in 0..max_x+1 {
      let char = if set.contains(&(x, y)) { "#" } else {"."};
      print!("{}", char)
    }
    println!("")
  }
}

fn fold(coords: HashSet<(i64, i64)>, (dx, dy): &(i64, i64)) -> HashSet<(i64, i64)> {
    println!("{:?}", (dx, dy));
    println!("{:?}", coords);

    HashSet::from_iter(
        coords
            .into_iter()
            .map(|(x, y)| {
              println!("{:?}", (x, y));


              let new_x = if x > *dx && *dx != 0 { 2*dx - x} else { x};
              let new_y = if y > *dy && *dy != 0 { 2*dy - y} else { y};
              println!("Becomes {:?}", (new_x , new_y));

              (new_x, new_y)
            },
          )
    )
}

fn parse_input(input: Vec<String>) -> (Vec<(i64, i64)>, Vec<(i64, i64)>) {
    let mut coordinates = vec![];
    let mut folds = vec![];
    let mut fold_instructions = false;

    for line in input.into_iter() {
        if line.is_empty() {
            fold_instructions = true;
            continue;
        }

        if fold_instructions {
            let ins = split_line(line, " ");
            let foldinfo = split_line(ins[2].clone(), "=");
            let pos = as_int(&foldinfo[1]);

            let instruction = if foldinfo[0] == "x" {
                (pos, 0)
            } else {
                (0, pos)
            };

            folds.push(instruction)
        } else {
            let coords = split_line(line, ",");

            coordinates.push((as_int(&coords[0]), as_int(&coords[1])));
        }
    }

    (coordinates, folds)
}
