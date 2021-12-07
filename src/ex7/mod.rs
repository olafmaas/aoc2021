use crate::generic::io::get_lines;
use crate::generic::parse::as_int;
use crate::generic::parse::split_line;



pub fn run() {
  let mut state = vec![0; 9];

  let line = get_lines("input/ex7/ex1").pop().unwrap();

  let input : Vec<i64> = split_line(line, ",").into_iter().map(|s| as_int(&s)).collect();

  part1(input.clone());
  part2(input)
}

fn part1(mut input: Vec<i64>) {

  input.sort();

  let middle = input.len()/2;

  println!("{:?}", input);

  let median = if middle % 2 == 0 {
    input[middle-1]
  } else {
    input[middle]
  };

  println!("median: {:?} fuel: {:?}", median, fuel(input, median));

}


fn part2(mut input: Vec<i64>) {

  let avg : f64 = input.iter().map(|s| *s as f64).sum::<f64>()/input.len() as f64;

  let avg_floor = avg.floor() as i64;
  let avg_ceil = avg.ceil() as i64;

  let fuel_floor = fuel2(input.clone(), avg_floor);
  let avg_ceil = fuel2(input, avg_ceil);

  println!("{:?}, {:?}", avg.round() as i64, fuel_floor.min(avg_ceil));
  // println!("median: {:?} fuel: {:?}", median, fuel(input, median));

}

fn fuel(input: Vec<i64>, pos: i64) -> i64 {
  input.iter().map(|crab| (crab - pos).abs()).sum()
}

fn fuel2(input: Vec<i64>, pos: i64) -> i64 {
  input.iter().map(|crab| (crab - pos).abs()).map(|fuel| (0..fuel+1).into_iter().sum::<i64>()).sum()
}