use std::string;
use std::collections::HashSet;
use itertools::Itertools;

use crate::generic::io::get_lines;
use crate::generic::parse::as_int;
use crate::generic::parse::split_line;

type Input = (Vec<String>, Vec<String>);
type MarkedDisplay = (isize, String);

pub fn run() {
  let input: Vec<Input> = get_lines("input/ex8/ex1").into_iter().map(parse_line).collect();

  println!("{:?}", input);

  part1(input.clone());
  part2(input);

}

fn parse_line(line: String) -> Input {
  let mut  line = split_line(line, "|");

  let output = split_line(line.pop().unwrap().trim().to_owned(), " ");
  let observations = split_line(line.pop().unwrap().trim().to_owned(), " ");

  (observations, output)
}

fn part1(input: Vec<Input>) {
  let count = input.into_iter().flat_map(|s| mark_all(s.1)).filter(|t| t.0 > -1).count();

  println!("count {:?}", count);
}


fn part2(input: Vec<Input>) {
  let sum : i64 = input.into_iter().map(solve).sum();

  println!("sum {:?}", sum);
}

fn solve((observations, output): Input) -> i64 {
  println!("{:?}", output);


  let marked_observed : Vec<MarkedDisplay> = observations.into_iter().map(mark1478).collect();
  let marked_output : Vec<MarkedDisplay> = output.into_iter().map(mark1478).collect();

  let result_string : String =   
    marked_output
    .into_iter()
    .map(|d| mark6(d, &marked_observed))
    .map(|d| mark9(d, &marked_observed))
    .map(|d| mark0(d, &marked_observed))
    .map(|d| mark3(d, &marked_observed))
    .map(|d| mark5(d, &marked_observed))
    .map(|d| mark2(d, &marked_observed))
    .map(|(i, _): MarkedDisplay| i.to_string())
    .collect_vec() 
    .join("");
  
  println!("{:?}", result_string);

  as_int(result_string.as_ref())
}

fn mark((mark, display): MarkedDisplay, condition: &dyn Fn(HashSet<String>) -> bool, on_true: isize ) -> MarkedDisplay {
  
  let set = HashSet::from_iter(split_line(display.clone(), "").into_iter().filter(|s| s != ""));
  let set2 = set.clone();



  if condition(set) && mark == -1 {
    println!("Marked {:?} as {:?}, set: {:?}", display, on_true, set2);

    (on_true, display)
  } else {
    (mark, display)
  }
}

fn mark6(display: MarkedDisplay, observed: &Vec<MarkedDisplay>) -> MarkedDisplay {
  let on = find_display(&observed, 1);

  mark(display, &|set: HashSet<String>| set.len() == 6 && on.intersection(&set).count() != 2, 6)
}

fn mark9(display: MarkedDisplay, observed:  &Vec<MarkedDisplay>) -> MarkedDisplay {
  let on = find_display(&observed, 4);

  mark(display, &|set: HashSet<String>| set.len() == 6 && on.intersection(&set).count()  == 4, 9)
}

fn mark0(display: MarkedDisplay, observed:  &Vec<MarkedDisplay>) -> MarkedDisplay {
  let on = find_display(&observed, 1);

  mark(display, &|set: HashSet<String>| set.len() == 6, 0)
}


fn mark3(display: MarkedDisplay, observed:  &Vec<MarkedDisplay>) -> MarkedDisplay {
  let on = find_display(&observed, 7);

  mark(display, &|set: HashSet<String>|set.len() == 5 && on.intersection(&set).count()  == 3, 3)
}

fn mark5(display: MarkedDisplay, observed:  &Vec<MarkedDisplay>) -> MarkedDisplay {
  let on = find_display(&observed, 4);

  mark(display, &|set: HashSet<String>| set.len() == 5 && on.intersection(&set).count() == 3, 5)
}

fn mark2(display: MarkedDisplay, observed:  &Vec<MarkedDisplay>) -> MarkedDisplay {
  mark(display, &|set: HashSet<String>| set.len() == 5, 2)
}



fn find_display(others: &Vec<(isize, String)>, num: isize) -> HashSet<String> {
  HashSet::from_iter(split_line(others.iter().find(|s| s.0 == num).unwrap().1.clone(), ""))
}


fn mark_all(displays: Vec<String>) -> Vec<(isize, String)>{
  displays.into_iter().map(mark1478).collect()
}

fn mark1478(display: String) -> (isize, String) {
  match display.len() {
    2 => (1, display),
    3 => (7, display),
    4 => (4, display),
    7 => (8, display),
    _ => (-1, display)
  }
}

