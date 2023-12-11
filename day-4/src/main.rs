use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashSet;
use std::sync::OnceLock;
use regex::Regex;
use std::collections::VecDeque;
use std::cmp::min;

const UNIVERSAL_ERROR_MESSAGE: &str = "Something went wrong. Help!";

struct GameFileIterator {
  reader: BufReader<File>,
  line: String
}

fn parse_file(path: &str) -> GameFileIterator {
  let file = File::open(path).expect(UNIVERSAL_ERROR_MESSAGE);
  let reader = BufReader::new(file);
  GameFileIterator { reader, line: String::new() }
}

#[derive(Debug,Default)]
struct GameCard {
  winning_numbers: Vec<u32>,
  selected_numbers: Vec<u32>
}

fn parse_numbers(input_str: &str) -> Vec<u32> {
  static NUMBERS_RE: OnceLock<Regex> = OnceLock::new();
  let numbers_re = NUMBERS_RE.get_or_init(|| Regex::new(r"(\d+)").expect(UNIVERSAL_ERROR_MESSAGE));
  numbers_re
    .captures_iter(input_str)
    .map(|captures| 
      captures
        .get(0)
        .map_or(0, |m| m.as_str().parse::<u32>().expect(UNIVERSAL_ERROR_MESSAGE)
      )
    ).collect()
}

fn parse_line(line: &String) -> GameCard {
  static PART_RE: OnceLock<Regex> = OnceLock::new();
  let part_regex = PART_RE.get_or_init(|| Regex::new(r"Card\s+(\d+)\D+([^|]+)\|(.+)").expect(UNIVERSAL_ERROR_MESSAGE));
  let (_, [_, winning_numbers_str, selected_numbers_str]) = part_regex.captures(&line).expect(UNIVERSAL_ERROR_MESSAGE).extract();

  GameCard {
    winning_numbers: parse_numbers(winning_numbers_str),
    selected_numbers: parse_numbers(selected_numbers_str),
  }
}

impl Iterator for GameFileIterator {
  type Item = GameCard;

  fn next(&mut self) -> Option<Self::Item> {
    self.line.clear();
    let result = self.reader.read_line(&mut self.line);
    match result {
      Err(_) => None,
      Ok(0) => None,
      Ok(_) => {
        Some(parse_line(&self.line))
      }
    }
  }
}

fn day_4_1(path: &str) -> u32 {
  parse_file(path)
    .map(|game_card| {
      let winning_numbers_hash: HashSet<&u32> = game_card.winning_numbers.iter().collect();
      let selected_numbers_hash: HashSet<&u32> = game_card.selected_numbers.iter().collect();
      let count = winning_numbers_hash
        .intersection(&selected_numbers_hash)
        .into_iter()
        .count();
      if count > 0 {
        1 << (count - 1)
      } else {
        0
      }
    })
    .fold(0, |sum, score| sum + score)
}

trait Part2State {
  fn record_game_result(&mut self, card_count: u32, score: u32);
  fn next(&mut self) -> u32;
}

impl Part2State for VecDeque<u32> {
  fn record_game_result(&mut self, total_copies: u32, score: u32) {
    let capacity_limit = min(self.len(), score.try_into().unwrap());
    for i in 0..capacity_limit {
      self[i] += total_copies;
    }
    for _ in capacity_limit..score.try_into().unwrap() {
      self.push_back(total_copies);
    }
  }

  fn next(&mut self) -> u32 {
    self.remove(0).unwrap_or(0)
  }
}

fn day_4_2(path: &str) -> u32 {
  let (total_score, _) = parse_file(path)
    .map(|game_card| {
      let winning_numbers_hash: HashSet<&u32> = game_card.winning_numbers.iter().collect();
      let selected_numbers_hash: HashSet<&u32> = game_card.selected_numbers.iter().collect();
      winning_numbers_hash
        .intersection(&selected_numbers_hash)
        .into_iter()
        .count()
        .try_into()
        .unwrap()
    })
    .fold((0, VecDeque::<u32>::new()), |(total_score, mut state), score| {
      let total_copies = state.next() + 1; // + 1 for the original copy
      state.record_game_result(total_copies, score);
      (total_score + total_copies, state)
    });
    total_score
}

fn main() {
  println!("{}", day_4_1("input"));
  println!("{}", day_4_2("input"));
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn day_4_1_handles_test_input() {
    assert_eq!(day_4_1("test_input"), 13);
  }

  #[test]
  fn day_4_2_handles_test_input() {
    assert_eq!(day_4_2("test_input"), 30);
  }
}
