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
  println!("{}", line);
  let re = Regex::new(r"Game (\d+):\s+(.*)").expect(UNIVERSAL_ERROR_MESSAGE);
  let (_, [id_str, handfuls_str]) = re.captures(line).unwrap().extract();
  let handfuls: Vec<Handful> = handfuls_str.split(';').map(|handful| {
    Regex::new(r"(\d+ \w+)")
      .expect(UNIVERSAL_ERROR_MESSAGE)
      .captures(handful)
      .unwrap()
      .iter()
      .fold(Handful::default(), |r, m| {
        println!("{:?}", m.map_or("", |m| m.as_str()));
        r
      })
  }).collect();

  return Game {
    id: id_str.parse().unwrap(),
    handfuls: handfuls.into(),
  }
}

fn day_2(input: &str) -> i32 {
  let file = fs::read_to_string(input).expect(UNIVERSAL_ERROR_MESSAGE);
  let games: Vec<Game> = file
    .lines()
    .map(|line| parse_line(line))
    .collect();

  println!("Game 0: {:?}, {:?}", games[0].id, games[0].handfuls);
  8
}

fn main() {
  day_2("input");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works_with_test_input() {
    assert_eq!(day_2("test_input"), 8);
  }
}
