use std::fs;
// use str::parse;
use regex::Regex;

enum State {
  Matching(usize),
  Match(),
  NoMatch()
}

struct StateMachine {
  matcher: Matcher,
  state: State,
}

struct Matcher {
  pattern: &'static str,
  output: i32
}

const ERROR_MESSAGE: &str = "Some weird error happened. Help!";

const NUMBER_REGEX: &str = r"(?:(\d)|(zero|one|two|three|four|five|six|seven|eight|nine))";

const OTHER_REGEX: &str = r"(\d|zero|one|two|three|four|five|six|seven|eight|nine).*(\d|zero|one|two|three|four|five|six|seven|eight|nine)";

const FIRST_REGEX: &str = r"(?:(\d)|(zero|one|two|three|four|five|six|seven|eight|nine))";

const LAST_REGEX: &str = r".*(?:(\d)|(zero|one|two|three|four|five|six|seven|eight|nine))";

const TOKENS: [Matcher; 18] = [
  Matcher { pattern: "1", output: 1 },
  Matcher { pattern: "2", output: 2 },
  Matcher { pattern: "3", output: 3 },
  Matcher { pattern: "4", output: 4 },
  Matcher { pattern: "5", output: 5 },
  Matcher { pattern: "6", output: 6 },
  Matcher { pattern: "7", output: 7 },
  Matcher { pattern: "8", output: 8 },
  Matcher { pattern: "9", output: 9 },
  Matcher { pattern: "one", output: 1 },
  Matcher { pattern: "two", output: 2 },
  Matcher { pattern: "three", output: 3 },
  Matcher { pattern: "four", output: 4 },
  Matcher { pattern: "five", output: 5 },
  Matcher { pattern: "six", output: 6 },
  Matcher { pattern: "seven", output: 7 },
  Matcher { pattern: "eight", output: 8 },
  Matcher { pattern: "nine", output: 9 },
];

fn next(s: StateMachine, c: char) -> StateMachine {
  match s.state {
    State::Matching(offset) => {
      if let Some(substring) = s.matcher.pattern.get(offset..) {
        if substring.chars().next() == Some(c) {
          if substring.len() == 1 {
            StateMachine {
              state: State::Match(),
              ..s
            }
          } else {
            StateMachine {
              state: State::Matching(offset + 1),
              ..s
            }
          }
        } else {
          StateMachine {
            state: State::NoMatch(),
            ..s
          }
        }
      } else {
        StateMachine {
          ..s
        }
      }
    },
    _ => s
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
