use std::fs;
use regex::Regex;

const UNIVERSAL_ERROR_MESSAGE: &str = "Something went wrong. Help!";

#[derive(Debug,Default)]
struct Handful {
  red: i32,
  green: i32,
  blue: i32,
}

#[derive(Debug)]
struct Game {
  id: i32,
  handfuls: Vec<Handful>,
}

fn parse_line(line: &str) -> Game {
  let re = Regex::new(r"Game (\d+):\s+(.*)").expect(UNIVERSAL_ERROR_MESSAGE);
  let (_, [id_str, handfuls_str]) = re.captures(line).unwrap().extract();
  let handfuls: Vec<Handful> = handfuls_str.split(';').map(|handful| {
    Regex::new(r"(\d+) (\w+)")
      .expect(UNIVERSAL_ERROR_MESSAGE)
      .captures_iter(handful)
      .fold(Handful::default(), |result, captures| {
        let (_, [count_str, color]) = captures.extract();
        match color {
          "red" => Handful { red: count_str.parse().expect(UNIVERSAL_ERROR_MESSAGE), ..result },
          "green" => Handful { green: count_str.parse().expect(UNIVERSAL_ERROR_MESSAGE), ..result },
          "blue" => Handful { blue: count_str.parse().expect(UNIVERSAL_ERROR_MESSAGE), ..result },
          other_string => panic!("I have no idea what color this is: {}", other_string),
        }
      })
  }).collect();

  return Game {
    id: id_str.parse().unwrap(),
    handfuls: handfuls.into(),
  }
}

fn day_2(input: &str) -> i32 {
  let file = fs::read_to_string(input).expect(UNIVERSAL_ERROR_MESSAGE);
  file
    .lines()
    .map(|line| parse_line(line))
    .fold(0, |result, game| 
      result + if game.handfuls.into_iter().all(|handful| handful.red < 13 && handful.green < 14 && handful.blue < 15) { game.id } else { 0 }
    )
}

fn main() {
  println!("{}", day_2("input"));
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works_with_test_input() {
    assert_eq!(day_2("test_input"), 8);
  }
}
