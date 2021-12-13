use itertools::enumerate;

use crate::generic::io::get_lines;
use crate::generic::parse::as_int;
use crate::generic::parse::split_line;

use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn run() {
    let iter = get_lines("input/ex11/ex1")
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

    println!("{:?}", map);
    // part1(map.clone());
    part2(map.clone());
}

fn print_map(map: &HashMap<(isize, isize), i64>){
  for y in 0..10 {
    for x in 0..10 {
      print!("{}", map.get(&(x, y)).unwrap())
    }
    println!("")
  }
  println!("")

}

fn part1(mut map: HashMap<(isize, isize), i64>) {

    let mut total_flashed = 0;

    for step in 0..100 {
        println!("Round {}", step);
        // print_map(&map);

        let mut flashed: HashSet<(isize, isize)> = HashSet::new();

        map.iter_mut().for_each(|(_, val)| *val += 1);

        let mut flashers: Vec<(isize, isize)> = map
            .iter()
            .filter(|(_, val)| **val > 9)
            .map(|(pos, _)| pos.clone())
            .collect_vec();

        while !flashers.is_empty() {
          // println!("flashers in round {}: {:?}", step, flashers);

          let new_flashers = flashers.iter().flat_map(adjacent_pos).collect_vec();
          // println!("new_flashers {:?}", new_flashers);


          flashed.extend(flashers.iter());

          new_flashers.iter().for_each(|pos| { map.entry(pos.clone()).and_modify(|val| *val += 1); });

          flashers = map
          .iter()
          .filter(|(pos, val)| **val > 9  && !flashed.contains(pos))
          .map(|(pos, _)| pos.clone())
          .collect_vec();
        }

        total_flashed += flashed.len();

        map.iter_mut().for_each(|(_, val)| { if *val > 9 { *val = 0; }} );
    }

    println!("Total flashed {}", total_flashed)
}

fn part2(mut map: HashMap<(isize, isize), i64>) {


  for step in 0..1000 {
      println!("Round {}", step);
      // print_map(&map);

      let mut flashed: HashSet<(isize, isize)> = HashSet::new();

      map.iter_mut().for_each(|(_, val)| *val += 1);

      let mut flashers: Vec<(isize, isize)> = map
          .iter()
          .filter(|(_, val)| **val > 9)
          .map(|(pos, _)| pos.clone())
          .collect_vec();

      while !flashers.is_empty() {
        // println!("flashers in round {}: {:?}", step, flashers);

        let new_flashers = flashers.iter().flat_map(adjacent_pos).collect_vec();
        // println!("new_flashers {:?}", new_flashers);


        flashed.extend(flashers.iter());

        new_flashers.iter().for_each(|pos| { map.entry(pos.clone()).and_modify(|val| *val += 1); });

        flashers = map
        .iter()
        .filter(|(pos, val)| **val > 9  && !flashed.contains(pos))
        .map(|(pos, _)| pos.clone())
        .collect_vec();
      }

      if flashed.len() == 100 {
        println!("Everyone flasehd in {}", step + 1);
        break;
      }

      map.iter_mut().for_each(|(_, val)| { if *val > 9 { *val = 0; }} );
  }

  println!("Done")
}



fn adjacent_pos((x, y): &(isize, isize)) -> Vec<(isize, isize)> {
    vec![
        (x + 1, y - 1),
        (x + 1, *y),
        (x + 1, y + 1),
        (x - 1, y - 1),
        (x - 1, *y),
        (x - 1, y + 1),
        (*x, y + 1),
        (*x, y - 1),
    ]
}
