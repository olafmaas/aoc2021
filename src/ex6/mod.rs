use crate::generic::io::get_lines;
use crate::generic::parse::as_int;
use crate::generic::parse::split_line;



pub fn run() {
  let mut state = vec![0; 9];

  let line = get_lines("input/ex6/ex1").pop().unwrap();

  split_line(line, ",").into_iter().map(|s| as_int(&s)).for_each(|i| {
    state[i as usize] += 1;
  });

  println!("{:?}", state);

  part1(state.clone(), 80);
  part1(state.clone(), 256);

}

fn part1(mut state: Vec<i64>, days: usize) {

  for day in 0..days {
    let parents = state.remove(0);
    state.push(parents);


    state[6] += parents;


    println!("day {:?}: {:?}", day, state)

  }

  println!("{:?}", state.iter().sum::<i64>());
}