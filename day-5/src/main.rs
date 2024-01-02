use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::ops::Range;
use std::collections::HashSet;
use std::pin::Pin;

const UNIVERSAL_ERROR_MESSAGE: &str = "Something unexpected happened. Help!";

enum ParseMode {
  Single(),
  Ranges(),
}

enum SplitRange {
  Outside(Range<i32>),
  Inside(Range<i32>),
}

fn split(seeds: Range<i32>, src_start: i32, length: i32) -> Vec<SplitRange> {
  let src_end = src_start + length - 1;
  let start_contained = seeds.contains(&src_start);
  let end_contained = seeds.contains(&src_end);
  match (start_contained, end_contained) {
    (true, true) => vec![SplitRange::Inside(seeds)],
    (true, false) => vec![SplitRange::Inside(seeds.start .. src_end), SplitRange::Outside(src_end .. seeds.end)],
    (false, true) => vec![SplitRange::Outside(seeds.start .. src_start), SplitRange::Inside(src_start .. seeds.end)],
    (false, false) => vec![SplitRange::Outside(seeds.start .. src_start), SplitRange::Inside(src_start .. src_end), SplitRange::Outside(src_end .. seeds.end)]
  }
}

#[derive(Debug)]
enum Projection {
  Mapped(Range<i32>),
  UnMapped(Range<i32>),
}

fn project(seeds: Range<i32>, map: (i32, i32, i32)) -> Vec<Projection> {
  let (dst_start, src_start, length) = map;
  println!("{} - {}", dst_start, src_start);
  let offset: i32 = (dst_start - src_start).try_into().unwrap();
  split(seeds, src_start, length)
    .iter()
    .map(|split_range| match split_range {
      SplitRange::Inside(seeds) => Projection::UnMapped(seeds.to_owned()),
      SplitRange::Outside(seeds) => Projection::Mapped(seeds.start + offset  .. seeds.end + offset),
    })
    .collect()
}

#[derive(Clone,Debug)]
struct Mapper {
  name: String,
  maps: Vec<(i32, i32, i32)>
}

impl Mapper {
  fn new() -> Mapper {
    Mapper {
      name: String::new(),
      maps: vec![],
    }
  }

  fn add_map(&mut self, dst_start: i32, src_start: i32, length: i32) {
    self.maps.push((dst_start, src_start, length));
  }

  fn map(&self, seed: Range<i32>) -> Vec<Range<i32>> {
    let mut mapped: HashSet<Range<i32>> = HashSet::new();
    let mut unmapped: HashSet<Range<i32>> = HashSet::from([seed]);

    for map in &self.maps {
      let unmapped_clone = unmapped.to_owned();
      let mut remove_unmapped = vec![];
      for seed in unmapped_clone {
        for projection in project(seed.start .. seed.end, *map) {
          match projection {
            Projection::Mapped(new_seed) => {
              remove_unmapped.push(seed.start .. seed.end);
              mapped.insert(new_seed);
            },
            _ => {}
          }
        }
      }
      for seed in remove_unmapped {
        unmapped.remove(&seed);
      }
    }
    
    vec![]
  }
}

#[derive(Debug)]
enum AlmanacItem {
  Seeds(Vec<Range<i32>>),
  Map(Mapper),
}

enum State {
  Initial(),
  ParsingMap(),
  Done(),
}

struct AlmanacIterator {
  reader: BufReader<File>,
  state: State,
  parse_mode: ParseMode,
}

fn parse_file(path: &str, parse_mode: ParseMode) -> AlmanacIterator {
  let file = File::open(path).expect(UNIVERSAL_ERROR_MESSAGE);
  let reader = BufReader::new(file);
  AlmanacIterator { reader, state: State::Initial(), parse_mode }
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
  Numbers(Vec<i32>)
}

fn parse_line(line: &str) -> Line {
  if line.trim().len() == 0 {
    return Line::Empty()
  }

  if line.contains("map:") {
    return Line::MapHeading(line.trim().into())
  }

  let numbers: Vec<i32> = line
    .split(|c| c > '9' || c < '0')
    .filter(|token| token.trim().len() > 0)
    .map(|token| token.parse::<i32>().unwrap())
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
              break match self.parse_mode {
                ParseMode::Single() => Some(AlmanacItem::Seeds(numbers.iter().map(|number| *number .. number + 1).collect())),
                ParseMode::Ranges() => Some(AlmanacItem::Seeds(numbers.chunks(2).map(|pair| pair[0] .. pair[0] + pair[1]).collect())),
              }
              
            },
            _ => {}
          }
        },
        State::Done() => {}
      }
    }
  }
}

fn day_5_1(path: &str) -> i32 {
  let mut almanac_iterator = parse_file(path, ParseMode::Single());
  let Some(AlmanacItem::Seeds(seeds)) = almanac_iterator.next() else { panic!("{}", UNIVERSAL_ERROR_MESSAGE) };
  let result = almanac_iterator.fold(seeds, |acc, item| {
    let AlmanacItem::Map(mapper) = item else { panic!("{} - {:?}", UNIVERSAL_ERROR_MESSAGE, item) };
    acc
      .into_iter()
      .map(|seed| mapper.map(seed))
      .flatten()
      .collect()
  });
  result.iter().map(|range| range.start).min().unwrap()
}

fn day_5_2(path: &str) -> i32 {
  let mut almanac_iterator = parse_file(path, ParseMode::Ranges());
  let Some(AlmanacItem::Seeds(seeds)) = almanac_iterator.next() else { panic!("{}", UNIVERSAL_ERROR_MESSAGE) };
  let result = almanac_iterator.fold(seeds, |acc, item| {
    let AlmanacItem::Map(mapper) = item else { panic!("{} - {:?}", UNIVERSAL_ERROR_MESSAGE, item) };
    println!("Using mapper: {:?}", mapper);
    acc
      .into_iter()
      .map(|seed| mapper.map(seed))
      .flatten()
      .collect()
  });
  result.iter().map(|range| range.start).min().unwrap()
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
    assert_eq!(day_5_2("test_input"), 46);
  }
}