use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::sync::OnceLock;
use regex::Regex;

const UNIVERSAL_ERROR_MESSAGE: &str = "Something unexpected happened. Help!";

#[derive(Debug,Default)]
struct Map {
  dst_start: u32,
  src_start: u32,
  length: u32
}

#[derive(Debug)]
enum AlmanacSection {
  Seeds(Vec<u32>),
  Map((String, u32, u32, u32)),
}

struct AlmanacIterator {
  reader: BufReader<File>,
  line: String
}

fn parse_file(path: &str) -> AlmanacIterator {
  let file = File::open(path).expect(UNIVERSAL_ERROR_MESSAGE);
  let reader = BufReader::new(file);
  AlmanacIterator { reader, line: String::new() }
}

impl AlmanacIterator {
  fn parse_seeds(seeds: &str) -> Vec<u32> {
    static SEEDS_RE: OnceLock<Regex> = OnceLock::new();
    let part_regex = SEEDS_RE.get_or_init(|| Regex::new(r"(\d+)").expect(UNIVERSAL_ERROR_MESSAGE));
    part_regex
      .captures_iter(seeds)
      .map(|captures| captures.get(1).expect(UNIVERSAL_ERROR_MESSAGE).as_str().parse::<u32>().expect(UNIVERSAL_ERROR_MESSAGE))
      .collect()
  }

  fn parse_almanac_section(&mut self) -> Option<AlmanacSection> {
    static HEADER_RE: OnceLock<Regex> = OnceLock::new();
    let part_regex = HEADER_RE.get_or_init(|| Regex::new(r"^(?:seeds: (.+)|(\S+) map:|)\s*$").expect(UNIVERSAL_ERROR_MESSAGE));
    let _ = &self.line.clear();
    let _ = self.reader.read_line(&mut self.line);

    let captures_option = part_regex.captures(&self.line);
    println!("{:?}: line: '{}'", captures_option, &self.line);
    let captures = captures_option.expect(UNIVERSAL_ERROR_MESSAGE);
    match (captures.get(1), captures.get(2)) {
      (Some(seeds), _) => Some(AlmanacSection::Seeds(AlmanacIterator::parse_seeds(seeds.as_str()))),
      (_, Some(map_name)) => Some(AlmanacSection::Map((String::from(map_name.as_str()), 1, 1, 1))),
      (None, None) => // empty line
    }
  }
}

impl Iterator for AlmanacIterator {
  type Item = AlmanacSection;
  
  fn next(&mut self) -> Option<Self::Item> {
    let section = self.parse_almanac_section();
    println!("Section 1: {:?}", section);
    let section = self.parse_almanac_section();
    println!("Section 2: {:?}", section);
    section
    // self.line.clear();
    // let result = self.reader.read_line(&mut self.line);
    // match result {
    //   Err(_) => None,
    //   Ok(0) => None,
    //   Ok(_) => {
    //     Some(self.parse_almanac_section())
    //   }
    // }
  }
}

fn day_5_1(path: &str) -> u32 {
  let mut almanac_iterator = parse_file(path);
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