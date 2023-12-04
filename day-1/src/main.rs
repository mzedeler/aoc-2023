use std::fs;
// use str::parse;
use regex::Regex;

const OTHER_REGEX: &str = r"(?<first>\d|zero|one|two|three|four|five|six|seven|eight|nine).*(?<last>\d|zero|one|two|three|four|five|six|seven|eight|nine)?.*?$";

const NUMBER_REGEX: &str = r"(?<digit>\d)(?<number>zero|one|two|three|four|five|six|seven|eight|nine)";

fn parse_number(number: &str) -> Option<i32> {
  match number {
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
    (digit) => digit.parse().ok()
  }
}

fn parse_line(line: &str) -> (i32, i32) {
  let re = Regex::new(OTHER_REGEX).unwrap();
  println!("Test line: {}", line);
  let captures = re.captures(line).unwrap();
  let first_parsed = parse_number(captures.name("first").map_or("", |m| m.as_str()));
  let last_parsed = parse_number(captures.name("last").map_or("", |m| m.as_str()));
  println!("T: {:?} {:?}", first_parsed, last_parsed);
  (first_parsed.unwrap(), last_parsed.unwrap_or(first_parsed.unwrap()))
}

fn day_1(path: &str) -> i32 {
  let file = fs::read_to_string(path).expect("ok");
  file
    .lines()
    .fold(0, |acc, line| {
      let (first, second) = parse_line(&line);
      println!("{}, {}", first, second);
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
