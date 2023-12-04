use std::fs;
use regex::Regex;

const FIRST_REGEX: &str = r"(?:(?<digit>\d)|(?<number>zero|one|two|three|four|five|six|seven|eight|nine))";
const LAST_REGEX: &str = r".*(?:(?<digit>\d)|(?<number>zero|one|two|three|four|five|six|seven|eight|nine))";

fn parse_number(regex: Box<Regex>, line: &str) -> Option<i32> {
  match regex.captures(line) {
    Some(captures) => if let Some(digit) = captures.name("digit") {
      digit.as_str().parse().ok()
    } else if let Some(number) = captures.name("number") {
      match number.as_str() {
        "zero" => Some(0),
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None
      }
    } else {
      None
    },
    None => None
  }
}

fn parse_line(first_re: Box<Regex>, last_re: Box<Regex>, line: &str) -> (i32, i32) {
  let first_parsed = parse_number(first_re, &line).unwrap();
  let last_parsed = parse_number(last_re, &line).unwrap_or(first_parsed);
  (first_parsed, last_parsed)
}

fn day_1(path: &str) -> i32 {
  let first_re = Regex::new(FIRST_REGEX).unwrap();
  let last_re = Regex::new(LAST_REGEX).unwrap();
  let file = fs::read_to_string(path).expect("ok");
  file
    .lines()
    .fold(0, |acc, line| {
      let (first, second) = parse_line(Box::new(first_re), Box::new(last_re), &line);
      acc + first * 10 + second
    })
}

fn main() {
  let r = day_1("input");
  println!("{}", r);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works_with_test_input() {
    assert_eq!(day_1("test_input"), 142);
  }

  #[test]
  fn it_works_with_test_input_2() {
    assert_eq!(day_1("test_input_2"), 51 + 29 + 53 + 19);
  }
}
