use std::fs;
// use str::parse;
use regex::Regex;

const ERROR_MESSAGE: &str = "Some weird error happened. Help!";

fn day_1(path: &str) -> i32 {
  let file = fs::read_to_string(path).expect("ok");
  let result = file
    .lines()
    .fold(0, |acc, line| {
      let re = Regex::new(r"^\D*(?<first>\d).*?(?<last>\d)?\D*$").unwrap();
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
