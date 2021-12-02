use crate::generic::io::get_lines;
use crate::generic::parse::as_int;
use crate::generic::parse::split_line;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Foward(i64),
    Down(i64),
    Up(i64)
}

pub fn run() {
  let instructions: Vec<Instruction> = get_lines("input/ex2/ex1")
      .into_iter()
      .map(parse)
      .collect();

  part1(instructions.clone());
  part2(instructions);
}

fn part1(instructions: Vec<Instruction>) {
  let (hor_pos, depth) = instructions
  .into_iter()
  .fold((0, 0), reduce);

  println!("{:?}, {:?}", (hor_pos, depth), hor_pos * depth);
}

fn reduce((hor_pos, depth): (i64, i64), instruction: Instruction) -> (i64, i64){
  match instruction {
    Instruction::Foward(i) => (hor_pos + i, depth),
    Instruction::Down(i) => (hor_pos, depth + i),
    Instruction::Up(i) => (hor_pos, depth - i)
  }
}

fn part2(instructions: Vec<Instruction>) {
  let (hor_pos, depth, aim) = instructions
  .into_iter()
  .fold((0, 0, 0), reduce_aim);

  print!("{:?}, {:?}", (hor_pos, depth, aim), hor_pos * depth);
}



fn reduce_aim((hor_pos, depth, aim): (i64, i64, i64), instruction: Instruction) -> (i64, i64, i64){
  match instruction {
    Instruction::Foward(i) => (hor_pos + i, depth + (aim * i), aim),
    Instruction::Down(i) => (hor_pos, depth, aim + i),
    Instruction::Up(i) => (hor_pos, depth, aim - i)
  }
}

fn parse(line: String) -> Instruction {
  let chunk = split_line(line, " ");

  let int = as_int(&chunk[1]);

  match chunk[0].as_str() {
      "forward" => Instruction::Foward(int),
      "down" => Instruction::Down(int),
      "up" => Instruction::Up(int),
      _ => panic!("Unkown Action")
  }
}