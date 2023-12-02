use std::fs;
// use str::parse;
use regex::Regex;

const ERROR_MESSAGE: &str = "Some weird error happened. Help!";

fn parse_line(line: &str) {
  let re = Regex::new(r"(?:(\d)|(one|two|three|four|five|six|seven|eight|nine))").unwrap();
  for captures in re.captures_iter(line) {
    let first = captures.get(1);
    let second = captures.get(2);
    let value = match (first, second) {
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
    };
    println!("Value: {}", value);
  }
  println!("--");
}

fn day_1(path: &str) -> i32 {
  let file = fs::read_to_string(path).expect("ok");
  let result = file
    .lines()
    .fold(0, |acc, line| {
      let re = Regex::new(r"^\D*(?<first>\d).*?(?<last>\d)?\D*$").unwrap();
      parse_line(&line);
      acc + match re.captures(&line) {
        None => 0,
        Some(captures) => {
          let first = captures.get(1).map_or("", |m| m.as_str());
          let last = captures.get(2).map_or(first, |m| m.as_str());
          let concatenated: String = "".to_owned() + first + last;
          concatenated.parse().expect(ERROR_MESSAGE)
        }
      }
    });
  return result;
}

fn main() {
  let r = day_1("input");
  println!("{}", r);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    assert_eq!(day_1("test_input"), 142);
  }
}
