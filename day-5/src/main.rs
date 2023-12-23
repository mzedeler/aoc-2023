use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::ops::Range;

const UNIVERSAL_ERROR_MESSAGE: &str = "Something unexpected happened. Help!";

#[derive(Clone,Debug)]
struct Mapper {
  name: String,
  maps: Vec<(u32, u32, u32)>
}

impl Mapper {
  fn new() -> Mapper {
    Mapper {
      name: String::new(),
      maps: vec![],
    }
  }

  fn add_map(&mut self, dst_start: u32, src_start: u32, length: u32) {
    self.maps.push((dst_start, src_start, length));
  }

  fn map(&self, seed: Range<u32>) -> Range<u32> {
    for (dst_start, src_start, length) in &self.maps {
      if seed.start >= *src_start && seed.start < src_start + length {
        let start = seed.start - src_start + dst_start;
        return start .. start + 1
      }
    }
    return seed
  }
}

#[derive(Debug)]
enum AlmanacItem {
  Seeds(Vec<Range<u32>>),
  Map(Mapper),
}

enum State {
  Initial(),
  ParsingMap(),
  Done(),
}

struct AlmanacIterator {
  reader: BufReader<File>,
  state: State
}

fn parse_file(path: &str) -> AlmanacIterator {
  let file = File::open(path).expect(UNIVERSAL_ERROR_MESSAGE);
  let reader = BufReader::new(file);
  AlmanacIterator { reader, state: State::Initial() }
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

enum Line {
  Empty(),
  MapHeading(String),
  Numbers(Vec<u32>)
}

fn parse_line(line: &str) -> Line {
  if line.trim().len() == 0 {
    return Line::Empty()
  }

  if line.contains("map:") {
    return Line::MapHeading(line.trim().into())
  }

  let numbers: Vec<u32> = line
    .split(|c| c > '9' || c < '0')
    .filter(|token| token.trim().len() > 0)
    .map(|token| token.parse::<u32>().unwrap())
    .collect();

  Line::Numbers(numbers)
}

impl Iterator for AlmanacIterator {
  type Item = AlmanacItem;
  
  fn next(&mut self) -> Option<Self::Item> {
    if let State::Done() = self.state {
      return None
    }

    let mut mapper = Mapper::new();
    loop {
      let Some(buffer) = self.next_line() else {
        self.state = State::Done();
        break Some(AlmanacItem::Map(mapper))
      };

      let line = parse_line(&buffer);

      match self.state {
        State::ParsingMap() => {
          match line {
            Line::Empty() => {
              break Some(AlmanacItem::Map(mapper))
            },
            Line::MapHeading(name) => {
              mapper.name = name
            },
            Line::Numbers(numbers) => {
              mapper.add_map(numbers[0], numbers[1], numbers[2]);
            }
          }
        },
        State::Initial() => {
          match line {
            Line::Numbers(numbers) => {
              self.next_line();
              self.state = State::ParsingMap();
              break Some(AlmanacItem::Seeds(numbers.iter().map(|number| *number .. number + 1).collect()))
            },
            _ => {}
          }
        },
        State::Done() => {}
      }
    }
  }
}

fn day_5_1(path: &str) -> u32 {
  let mut almanac_iterator = parse_file(path);
  let Some(AlmanacItem::Seeds(seeds)) = almanac_iterator.next() else { panic!("{}", UNIVERSAL_ERROR_MESSAGE) };
  let result = almanac_iterator.fold(seeds, |acc, item| {
    let AlmanacItem::Map(mapper) = item else { panic!("{} - {:?}", UNIVERSAL_ERROR_MESSAGE, item) };
    println!("Using mapper: {:?}", mapper);
    acc
      .into_iter()
      .map(|seed| mapper.map(seed))
      .collect()
  });
  result.iter().map(|range| range.start).min().unwrap()
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