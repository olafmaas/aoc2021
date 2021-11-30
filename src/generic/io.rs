use std::fs;

pub fn get_lines(file_name: &str) -> Vec<String> {
  return fs::read_to_string(file_name)
      .unwrap()
      .lines()
      .map(|s| s.to_owned())
      .collect();
}