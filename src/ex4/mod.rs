use core::num;
use std::collections::HashSet;

use crate::generic::io::get_lines;
use crate::generic::parse::as_int;
use crate::generic::parse::split_line;
use itertools::Itertools;

type Board = Vec<Vec<i64>>;

pub fn run() {
    let mut lines: Vec<String> = get_lines("input/ex4/ex1");

    let numbers_string = lines.remove(0);

    let numbers: Vec<i64> = split_line(numbers_string, ",")
        .into_iter()
        .map(|s| as_int(s.as_str()))
        .collect();

    let boards = parse_boards(lines);

    part1(numbers.clone(), boards.clone());
    part2(numbers, boards);
}

fn parse_boards(mut lines: Vec<String>) -> Vec<Board> {
    //First empty line
    lines.remove(0);

    let mut boards: Vec<Board> = vec![Vec::new()];

    for line in lines.into_iter() {
        if line.is_empty() {
            boards.push(Vec::new());
        } else {
            println!("line {:?}", line);
            let board_line = split_line(line, " ")
                .into_iter()
                .filter(|s| !s.is_empty())
                .map(|s| as_int(s.as_str().trim()))
                .collect();

            boards.last_mut().unwrap().push(board_line);
        }
    }

    boards
}

fn part1(numbers: Vec<i64>, boards: Vec<Board>) {
    println!("{:?}", boards);

    let (winning_board, drawn, last) = play_bingo(numbers, boards);

    print!("{:?} {:?}", winning_board, drawn);

    let sum_of_unmarked: i64 = winning_board
        .iter()
        .map(|row| row.iter().filter(|num| !drawn.contains(num)).sum::<i64>())
        .sum();

    println!("{:?}", sum_of_unmarked * last)
}

fn play_bingo(numbers: Vec<i64>, boards: Vec<Board>) -> (Board, HashSet<i64>, i64) {
    let height = boards[0].len();
    let width = boards[0][0].len();

    let mut set = HashSet::new();

    for number in numbers.into_iter() {
        set.insert(number);

        let result = play_round(&set, &boards, width, height);

        match result {
            Some((board, _)) => return (board, set, number),
            None => (),
        }
    }

    panic!("No winning Boards")
}

fn part2(numbers: Vec<i64>, boards: Vec<Board>) {
    println!("{:?}", boards);

    let (losing_board, drawn, last) = play_bingo_last_winner(numbers, boards);

    print!("{:?} {:?}", losing_board, drawn);

    let sum_of_unmarked: i64 = losing_board
        .iter()
        .map(|row| row.iter().filter(|num| !drawn.contains(num)).sum::<i64>())
        .sum();

    println!("{:?}", sum_of_unmarked * last)
}

fn play_bingo_last_winner(numbers: Vec<i64>, mut boards: Vec<Board>) -> (Board, HashSet<i64>, i64) {
    let height = boards[0].len();
    let width = boards[0][0].len();

    let mut set = HashSet::new();

    for number in numbers.into_iter() {
        set.insert(number);

        let winners = play_round_multi_win(&set, &boards, width, height);

        if boards.len() == 1 && winners.len() == 1 {
            return (boards.pop().unwrap(), set, number);
        } else {
            let winner_set: HashSet<usize> =
                HashSet::from_iter(winners.iter().map(|(_, pos)| *pos));

            boards = boards
                .into_iter()
                .enumerate()
                .filter(|(pos, _)| !winner_set.contains(pos))
                .map(|(_, board)| board)
                .collect()
        }
    }

    panic!("No winning Boards")
}

fn play_round(
    numbers: &HashSet<i64>,
    boards: &Vec<Board>,
    width: usize,
    height: usize,
) -> Option<(Board, usize)> {
    boards
        .iter()
        .find_position(|board| has_bingo(*board, numbers, width, height))
        .map(|(pos, board)| (board.clone(), pos))
}

fn play_round_multi_win(
    numbers: &HashSet<i64>,
    boards: &Vec<Board>,
    width: usize,
    height: usize,
) -> Vec<(Board, usize)> {
    boards
        .iter()
        .enumerate()
        .filter(|(_, board)| has_bingo(*board, numbers, width, height))
        .map(|(pos, board)| (board.clone(), pos))
        .collect()
}

fn has_bingo(board: &Board, numbers: &HashSet<i64>, width: usize, height: usize) -> bool {
    // let column_win = ;
    let row_win = board
        .iter()
        .any(|row| row.iter().all(|num| numbers.contains(num)));
    let column_win = (0..width)
        .map(|col| board.iter().map(|row| row[col]).collect_vec())
        .any(|col| col.iter().all(|num| numbers.contains(num)));

    column_win || row_win
}
