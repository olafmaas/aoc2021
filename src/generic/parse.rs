pub fn as_int(input: &str) -> i64 {
  return input.parse().unwrap();
}

pub fn split_line(line: String, on: &str) -> Vec<String> {
  return line.split(on).map(|s| s.to_owned()).collect();
}