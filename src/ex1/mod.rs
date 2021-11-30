use crate::generic::io::get_lines;
use crate::generic::parse::split_line;
use itertools::Itertools;

pub fn run() {
    let lines = get_lines("input/ex1/test");

    println!(
        "{:?}",
        lines
            .into_iter()
            .map(|line| split_line(line, " "))
            .collect_vec()
    )
}
