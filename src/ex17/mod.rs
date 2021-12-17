use crate::generic::io::get_lines;
use crate::generic::parse::as_int;
use crate::generic::parse::split_line;

use itertools::Itertools;
use std::collections::HashSet;

use std::ops::Range;

pub fn run() {
    let (x_range, y_range) = parse_input(get_lines("input/ex17/ex1"));

    println!("{:?}", x_range);
    println!("{:?}", y_range);

    let y_min = y_range.start.min(y_range.end);

    let y_iter = y_min.min(-y_min)..-y_min.max(y_min);

    println!("{:?}", y_iter);

    let res = (0..x_range.end+1)
        .cartesian_product(y_iter)
        .flat_map(|(x, y)| solve(x, y, (&x_range, &y_range)))
        .collect_vec();

    println!("max y: {}", res.iter().max().unwrap());
 
    println!("distinct vels: {}", res.len());
}

fn solve(mut vx: i64, mut vy: i64, (x_range, y_range): (&Range<i64>, &Range<i64>)) -> Vec<i64> {
    // println!("{}, {}", vx, vy);

    let mut x = 0;
    let mut y = 0;

    let mut max_y = i64::MIN;

    while y > y_range.start.min(y_range.end) {
        x += vx;
        y += vy;

        vy -= 1;

        if vx != 0 {
            vx -= vx.signum() * 1
        }

        max_y = max_y.max(y);

        if x_range.contains(&x) && y_range.contains(&y) {
            println!("{}", max_y);
            return vec![max_y];
        }
    }

    vec![]
}

fn parse_input(mut lines: Vec<String>) -> (Range<i64>, Range<i64>) {
    let line = lines.pop().unwrap();

    let parts = split_line(line, ",");

    let y_range_string = parts[1].replace(" y=", "");
    let x_range_string = parts[0].replace("target area: x=", "");

    (parse_range(x_range_string), parse_range(y_range_string))
}

fn parse_range(line: String) -> Range<i64> {
    let ints = split_line(line, "..");

    as_int(&ints[0])..as_int(&ints[1])+1
}
