use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

const UNIVERSAL_ERROR_MESSAGE: &str = "Something unexpected happened. Help!";

#[derive(Debug)]
enum AlmanacItem {
  Seeds(Vec<u32>),
  Map((u32, u32, u32)),
}

struct AlmanacIterator {
  reader: BufReader<File>,
  seeds_emitted: bool
}

fn parse_file(path: &str) -> AlmanacIterator {
  let file = File::open(path).expect(UNIVERSAL_ERROR_MESSAGE);
  let reader = BufReader::new(file);
  AlmanacIterator { reader, seeds_emitted: false }
}

impl AlmanacIterator {
  fn next_line(&mut self) -> Option<String> {
    let mut buffer = String::new();

    let read_length = &self
      .reader
      .read_line(&mut buffer)
      .expect(UNIVERSAL_ERROR_MESSAGE);

    match read_length {
      0 => None,
      _ => Some(buffer)
    }
  }
}

impl Iterator for AlmanacIterator {
  type Item = AlmanacItem;
  
  fn next(&mut self) -> Option<Self::Item> {
    loop {
      let buffer = Some(self.next_line())?;

      let numbers: Vec<u32> = buffer.unwrap()
        .split(|c| c > '9' || c < '0')
        .filter(|token| token.trim().len() > 0)
        .map(|token| token.parse::<u32>().unwrap())
        .collect();

      if numbers.len() > 0 {
        if self.seeds_emitted {
          println!("Numbers before emit: {:?}", numbers);
          break Some(AlmanacItem::Map((numbers[0], numbers[1], numbers[2])))
        } else {
          self.seeds_emitted = true;
          break Some(AlmanacItem::Seeds(numbers))
        }  
      }
    }
  }
}

fn day_5_1(path: &str) -> u32 {
  let mut almanac_iterator = parse_file(path);
  println!("-> {:?}", almanac_iterator.next());
  println!("-> {:?}", almanac_iterator.next());
  println!("-> {:?}", almanac_iterator.next());
  println!("-> {:?}", almanac_iterator.next());
  println!("-> {:?}", almanac_iterator.next());
  1
}

fn day_5_2(path: &str) -> u32 {
  1
}

fn main() {
  println!("{}", day_5_1("input"));
  println!("{}", day_5_2("input"));
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn day_5_1_handles_test_input() {
    assert_eq!(day_5_1("test_input"), 35);
  }

  #[test]
  fn day_5_2_handles_test_input() {
    assert_eq!(day_5_2("test_input"), 1);
  }
}