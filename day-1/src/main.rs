use std::fs;
// use str::parse;
use regex::Regex;

struct StateMachine<T: AsRef<str>> {
  pattern: T,
  output: i32
}

const ERROR_MESSAGE: &str = "Some weird error happened. Help!";

const NUMBER_REGEX: &str = r"(?:(\d)|(zero|one|two|three|four|five|six|seven|eight|nine))";

const OTHER_REGEX: &str = r"(\d|zero|one|two|three|four|five|six|seven|eight|nine).*(\d|zero|one|two|three|four|five|six|seven|eight|nine)";

const FIRST_REGEX: &str = r"(?:(\d)|(zero|one|two|three|four|five|six|seven|eight|nine))";

const LAST_REGEX: &str = r".*(?:(\d)|(zero|one|two|three|four|five|six|seven|eight|nine))";

const TOKENS: [StateMachine<&'static str>; 2] = [StateMachine { pattern: "1", output: 1 }, StateMachine { pattern: "1", output: 1 }];

fn next<'a>(s: StateMachine<&'a str>, c: char) -> Option<StateMachine<&'a str>> {
  if s.pattern.len() == 0 {
    panic!("{}", ERROR_MESSAGE)
  }
  if s.pattern.chars().next() == Some(c) {
    Some(StateMachine {
      pattern: s.pattern.get(1..).unwrap(),
      output: s.output
    })
  } else {
    None
  }
}

fn parse_line(line: &str) -> Vec<i32> {
  // let mut result: Vec<i32> = vec!();

  let re2 = Regex::new(OTHER_REGEX).unwrap();
  let result = re2.captures(&line);
  if let Some(captures) = result {
    println!("C: {}/{:?} - {:?}", &line, captures, captures.get(2).map_or("", |m| m.as_str()));
  }
  let re = Regex::new(NUMBER_REGEX).unwrap();
  re.captures_iter(line).map(|captures| {
    let first = captures.get(1);
    let second = captures.get(2);
    match (first, second) {
      (Some(digit), _) => digit.as_str().parse().unwrap(),
      (_, Some(number)) => match number.as_str() {
        "zero" => 0,
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => panic!("{}", ERROR_MESSAGE),
      },
      (None, None) => panic!("{}", ERROR_MESSAGE),
    }
  }).collect()
}

fn day_1(path: &str) -> i32 {
  let file = fs::read_to_string(path).expect("ok");
  file
    .lines()
    .fold(0, |acc, line| {
      let v = parse_line(&line);
      println!("{}: {:?}", &line, &v);
      acc + match v.len() {
        1 => v[0] * 10 + v[0],
        _ => v[0] * 10 + v[v.len() - 1],
      }
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
