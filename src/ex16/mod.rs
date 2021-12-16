use crate::generic::io::get_lines;

#[derive(Debug, Clone)]
enum Package {
    Literal(isize, (isize, isize)),
    Operator(Vec<Package>, (isize, isize)),
}

pub fn run() {
    let binary_string = parse_input(get_lines("input/ex16/ex1"));

    println!("{}", binary_string);

    let (package, _) = parse_package(&binary_string, 0);

    println!("{:?}", package);

    println!("Version Sum: {:?}", version_sum(&package));

    println!("interpret: {:?}", interpret(&package));
}

fn version_sum(package: &Package) -> isize {
    match package {
        Package::Literal(_, (_, version)) => *version,
        Package::Operator(packages, (_, version)) => {
            *version + packages.iter().map(version_sum).sum::<isize>()
        }
    }
}

fn interpret(package: &Package) -> isize {
    match package {
        Package::Literal(value, (_, _)) => *value,
        Package::Operator(packages, (ty, _)) => match ty {
            0 => packages.iter().map(interpret).sum(),
            1 => packages.iter().map(interpret).fold(1, |acc, i| acc * i),
            2 => packages.iter().map(interpret).min().unwrap(),
            3 => packages.iter().map(interpret).max().unwrap(),
            5 | 6 | 7 => {
                let lhs = interpret(&packages[0]);
                let rhs = interpret(&packages[1]);

                operator(*ty, lhs, rhs)
            }
            c => panic!("Unknown Opcode {}", c),
        },
    }
}

fn operator(ty: isize, lhs: isize, rhs: isize) -> isize {
    let bool = match ty {
        5 => lhs > rhs,
        6 => lhs < rhs,
        7 => lhs == rhs,
        c => panic!("Unknown Opcode {}", c),
    };

    if bool {
        1
    } else {
        0
    }
}

fn parse_package(binary_string: &String, pos: usize) -> (Package, usize) {
    let version: isize = bin_to_isize(&binary_string[pos..pos + 3]);
    let type_id: isize = bin_to_isize(&binary_string[pos + 3..pos + 6]);

    if type_id == 4 {
        parse_literal_package(binary_string, pos + 6, version)
    } else {
        parse_operator_package(binary_string, pos + 6, version, type_id)
    }
}

fn parse_operator_package(
    binary_string: &String,
    start_pos: usize,
    version: isize,
    type_id: isize,
) -> (Package, usize) {
    let length_type = bin_to_isize(&binary_string[start_pos..start_pos + 1]);

    if length_type == 0 {
        parse_fixed_length(binary_string, start_pos + 1, version, type_id)
    } else {
        parse_variable_length(binary_string, start_pos + 1, version, type_id)
    }
}

fn parse_fixed_length(
    binary_string: &String,
    start_pos: usize,
    version: isize,
    type_id: isize,
) -> (Package, usize) {
    let bit_len = bin_to_isize(&binary_string[start_pos..start_pos + 15]);

    let mut pos = start_pos + 15;

    let mut packages: Vec<Package> = vec![];

    while (pos - (start_pos + 15)) < bit_len as usize {
        let (package, new_pos) = parse_package(binary_string, pos);

        pos = new_pos;

        packages.push(package);
    }

    (Package::Operator(packages, (type_id, version)), pos)
}

fn parse_variable_length(
    binary_string: &String,
    start_pos: usize,
    version: isize,
    type_id: isize,
) -> (Package, usize) {
    let package_len = bin_to_isize(&binary_string[start_pos..start_pos + 11]);

    let mut pos = start_pos + 11;

    let mut packages: Vec<Package> = vec![];

    for i in 0..package_len {
        let (package, new_pos) = parse_package(binary_string, pos);

        pos = new_pos;

        packages.push(package);
    }

    (Package::Operator(packages, (type_id, version)), pos)
}

fn parse_literal_package(
    binary_string: &String,
    start_pos: usize,
    version: isize,
) -> (Package, usize) {
    let mut value_string = "".to_owned();

    let mut pos = start_pos;

    let mut is_last = false;

    while !is_last {
        let res = &binary_string[pos..pos + 1];
        let val = &binary_string[pos + 1..pos + 5];

        pos += 5;

        is_last = res.chars().nth(0).unwrap() == '0';

        value_string.push_str(val)
    }

    let value = bin_to_isize(value_string.as_str());

    (Package::Literal(value, (4, version)), pos)
}

fn bin_to_isize(string: &str) -> isize {
    usize::from_str_radix(string, 2).unwrap() as isize
}

fn parse_input(mut input: Vec<String>) -> String {
    let mut result = "".to_owned();

    let line = input.pop().unwrap();

    for c in line.chars() {
        let append = match c {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
            c => panic!("Not a hex, {}", c),
        };

        result.push_str(append)
    }

    result
}
