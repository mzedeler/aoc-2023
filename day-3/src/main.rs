use std::fs;

const UNIVERSAL_ERROR_MESSAGE: &str = "Something went wrong. Help!";

#[derive(Debug,PartialEq)]
enum Cell {
  Digit(u32),
  Part(char),
  Empty()
}

#[derive(Debug,PartialEq,Default)]
struct SchemaNumber {
  row_number: usize,
  col_start: usize,
  col_end: usize,
  number: u32
}

#[derive(Debug,PartialEq)]
enum State {
  Initial(),
  Empty(),
  Part(),
  ParsingSchemaNumber(SchemaNumber)
}

fn parse_line(line: &str) -> Vec<Cell> {
  line.chars().map(|c| 
    match c {
      '.' => Cell::Empty(),
      '0' ..= '9' => Cell::Digit(c.to_digit(10).unwrap()),
      _ => Cell::Part(c)
    }
  ).into_iter().collect()
}

fn parse_file(path: &str) -> Vec<Vec<Cell>> {
  let file = fs::read_to_string(path).expect(UNIVERSAL_ERROR_MESSAGE);
  file
    .lines()
    .map(|line| parse_line(line))
    .collect()
}

fn day_3_1(path: &str) -> u32 {
  let schematic = parse_file(path);
  let mut state = State::Initial();
  let mut parts: Vec<(usize, usize)> = vec![];
  let mut numbers = vec![];
  let mut number_references: Vec<Vec<Option<usize>>> = vec![vec![None; schematic[0].len()]; schematic.len()];

  let mut store_number =  |state: &State, row_number: usize| {
    if let State::ParsingSchemaNumber(schema_number) = state {
      numbers.push(schema_number.number);
      for col_number in schema_number.col_start ..= schema_number.col_end {
        number_references[row_number][col_number] = Some(numbers.len());
      }
    }
  };

  for row_number in 0.. schematic.len() {
    for col_number in 0.. schematic[row_number].len() {
      let cell = &schematic[row_number][col_number];
      match cell {
        Cell::Digit(digit) => {
          if let State::ParsingSchemaNumber(ref mut schema_number) = state {
            schema_number.number = schema_number.number * 10 + digit;
            schema_number.col_end = col_number;
          } else {
            let schema_number = SchemaNumber { 
              row_number,
              col_start: col_number,
              col_end: col_number,
              number: *digit
            };
            state = State::ParsingSchemaNumber(schema_number);
          }
        },
        Cell::Empty() => {
          store_number(&state, row_number);
          state = State::Empty();
        },
        Cell::Part(_) => {
          store_number(&state, row_number);
          parts.push((row_number, col_number));
          state = State::Part();
        }
      }
    }
  }
  println!("{:?} {:?} {:?}", numbers, number_references, parts);
  4361
}

fn main() {
    println!("{}", day_3_1("input"));
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn day_3_1_handles_test_input() {
    assert_eq!(day_3_1("test_input"), 4361);
  }
}