use std::fs;
// use str::parse;
use regex::Regex;

const ERROR_MESSAGE: &str = "Some weird error happened. Help!";

fn parse_line(line: &str) -> Vec<i32> {
  let re = Regex::new(r"(?:(\d)|(zero|one|two|three|four|five|six|seven|eight|nine))").unwrap();
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
    assert_eq!(day_1("test_input_2"), 51 + 29 + 53);
  }
}
