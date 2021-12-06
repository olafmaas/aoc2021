use crate::generic::io::get_lines;
use crate::generic::parse::as_int;
use crate::generic::parse::split_line;
use geo::algorithm::line_intersection::{line_intersection, LineIntersection};
use geo::{Coordinate, Line};
use itertools::Itertools;
use std::collections::HashMap;

type Line2D = Line<i64>;

pub fn run() {
    let parsed_lines: Vec<Line2D> = get_lines("input/ex5/ex1")
        .into_iter()
        .map(|s| as_line(s))
        .collect();

    part1_kiss(parsed_lines.clone());
    part2_kiss(parsed_lines.clone());

}




fn as_line(input_line: String) -> Line2D {
    let points = split_line(input_line, " -> ");

    let start_coordinates = split_line(points[0].clone(), ",");
    let end_coordinates = split_line(points[1].clone(), ",");

    let (x, y) = (as_int(&start_coordinates[0]), as_int(&start_coordinates[1]));
    let (x_end, y_end) = (as_int(&end_coordinates[0]), as_int(&end_coordinates[1]));

    Line::new(
        Coordinate {
            x: x,
            y: y,
        },
        Coordinate {
            x: x_end,
            y: y_end,
        },
    )
}

fn part1_kiss(lines: Vec<Line2D>) {
    let mut points: HashMap<(i64, i64), usize> = HashMap::new();

    let filtered_lines : Vec<Line2D> = lines
        .into_iter()
        .filter(|s| s.dx() == 0|| s.dy() == 0)
        .collect();

    println!("{:?}", filtered_lines.len());

    filtered_lines
        .into_iter()
        .flat_map(|l| points_for(&l))
        .into_iter()
        .for_each(|point| { points.insert(point, points.get(&point).unwrap_or(&0) + 1);});

    let overlap = points.values().filter(|count| **count > 1).count();

    println!("{:?}", points);
    println!("{:?}", overlap);
}

fn part2_kiss(lines: Vec<Line2D>) {
  let mut points: HashMap<(i64, i64), usize> = HashMap::new();

  let filtered_lines : Vec<Line2D> = lines
      .into_iter()
      .collect();

  println!("{:?}", filtered_lines.len());

  filtered_lines
      .into_iter()
      .flat_map(|l| points_for(&l))
      .into_iter()
      .for_each(|point| { points.insert(point, points.get(&point).unwrap_or(&0) + 1);});

  let overlap = points.values().filter(|count| **count > 1).count();

  println!("{:?}", points);
  println!("{:?}", overlap);
}

fn points_for(line: &Line2D) -> Vec<(i64, i64)> {
  let len = line.dx().abs().max(line.dy().abs());

  let dx_sign = line.dx().signum();
  let dy_sign = line.dy().signum();

  (0..(len+1)).into_iter().map(|step| ((line.start.x + dx_sign * step), (line.start.y + dy_sign * step))).collect() 
}

// fn part1(lines: Vec<Line2D>) {
//     let mut points: HashSet<(i64, i64)> = HashSet::new();

//     let filtered_lines : Vec<Line2D> = lines
//         .into_iter()
//         .filter(|s| s.dx() == 0.0 || s.dy() == 0.0)
//         .collect();

//     println!("{:?}", filtered_lines.len());

//     filtered_lines
//         .into_iter()
//         .tuple_combinations()
//         .for_each(|tuple| reduce(&mut points, tuple));

//     println!("{:?}", points.len());
// }

// fn reduce(points: &mut HashSet<(i64, i64)>, (line1, line2): (Line2D, Line2D)) {
//     match line_intersection(line1, line2) {
//         None => (),
//         Some(LineIntersection::SinglePoint { intersection, .. }) => {
//             let (x, y) = intersection.x_y();

//             points.insert((x as i64, y as i64));
//         }
//         Some(LineIntersection::Collinear { intersection }) => {
//             if intersection.dx() == 0.0 {
//                 let x = intersection.start.x as i64;
//                 let intersection_points: Vec<(i64, i64)> = (intersection.start.y as i64
//                     ..intersection.end.y as i64 + 1)
//                     .into_iter()
//                     .map(|y| (x, y))
//                     .collect();

//                 points.extend(intersection_points);
//             } else {
//                 let y = intersection.start.y as i64;
//                 let intersection_points: Vec<(i64, i64)> = (intersection.start.x as i64
//                     ..intersection.end.x as i64 + 1)
//                     .into_iter()
//                     .map(|x| (x, y))
//                     .collect();

//                 points.extend(intersection_points);
//             }
//         }
//     }
// }
