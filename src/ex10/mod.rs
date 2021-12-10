use itertools::Itertools;

use crate::generic::io::get_lines;

enum Error{
  Incomplete(usize),
  Corrupted(usize)
}


pub fn run() {
    let input: Vec<String> = get_lines("input/ex10/ex1");

    part1(input.clone());
    part2(input);

}

fn part1(input: Vec<String>) {
  let line_values = input
  .into_iter()
  .flat_map(|l| as_value(parse(l)))
  .collect_vec();

  println!("{:?}, {:?}", line_values, line_values.iter().sum::<usize>())
}

fn as_value(res: Result<(), Error>) -> Vec<usize> {
  match res {
    Err(Error::Corrupted(i)) => vec![i],
    _ => vec![]
  }
}

fn part2(input: Vec<String>) {
  let mut line_values = input
  .into_iter()
  .flat_map(|l| as_value_incomplete(parse(l)))
  .collect_vec();

  println!("{:?}", line_values);

  line_values.sort();
  let mid_pos = line_values.len()/2;

  println!("{:?}", line_values[mid_pos])
}

fn parse(input: String) -> Result<(), Error> {
  let mut stack = Vec::new();

  for char in input.chars() {

    match char {
      '<' | '[' | '{' | '(' => stack.push(char),
      '>' | ']' | '}' | ')' => compare(&mut stack, inverse(char), char)?,
      c => panic!("Uknown char {}", c)
    };
  }

  if stack.is_empty() {
    return Ok(());

  } 

  Err(build_incomplete(stack))
}

fn inverse(c: char) -> char {
  match c {
    '>' => '<',
    ']' => '[',
    '}' => '{',
    ')' => '(',
    c => panic!("Uknown char {}", c)
  }
}

fn as_value_incomplete(res: Result<(), Error>) -> Vec<usize> {
  match res {
    Err(Error::Incomplete(i)) => vec![i],
    _ => vec![]
  }
}

fn compare(stack: &mut Vec<char>, expected: char, current: char) -> Result<(), Error> {
  let found = stack.pop().unwrap();

  if found != expected {
    let score = match current {
      ')' => 3,
      ']' => 57,
      '}' => 1197,
      '>' => 25137,
      c => panic!("Uknown char {}", c)
    };

    return Err(Error::Corrupted(score))
  }

  Ok(())
}

fn build_incomplete(mut stack: Vec<char>) -> Error {
  let score = stack
  .into_iter()
  .rev()
  .fold(0, |acc, c| {
    let score : usize = match c {
      '(' => 1,
      '[' => 2,
      '{' => 3,
      '<' => 4,
      c => panic!("Uknown char {}", c)
    };

    acc*5 + score
  });

  Error::Incomplete(score)
}