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

struct CollectParts {
  parts: Vec<(usize, usize, char)>,
  numbers: Vec<u32>,
  number_references: Vec<Vec<Option<usize>>>
}

fn collect_parts(path: &str) -> CollectParts {
  let schematic = parse_file(path);
  let mut state = State::Initial();
  let mut parts: Vec<(usize, usize, char)> = vec![];
  let mut numbers = vec![];
  let mut number_references: Vec<Vec<Option<usize>>> = vec![vec![None; schematic[0].len()]; schematic.len()];

  let mut store_number =  |state: &State, row_number: usize| {
    if let State::ParsingSchemaNumber(schema_number) = state {
      numbers.push(schema_number.number);
      for col_number in schema_number.col_start ..= schema_number.col_end {
        number_references[row_number][col_number] = Some(numbers.len() - 1);
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
        Cell::Part(c) => {
          store_number(&state, row_number);
          parts.push((row_number, col_number, *c));
          state = State::Part();
        }
      }
    }
    store_number(&state, row_number);
    state = State::Empty();
  }

  CollectParts { parts, numbers, number_references }
}

fn day_3_1(path: &str) -> u32 {
  let CollectParts { parts, numbers, number_references } = collect_parts(path);

  let selected_number_references: Vec<usize> = parts
    .iter()
    .map(|(row_number, col_number, _)| {
      static EMPTY: Vec<Option<usize>> = vec![];
      static OFFSETS: [(isize, isize); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
      OFFSETS
        .map(|(row_offset, col_offset)| {
          let peek_row_number = row_number.checked_add_signed(row_offset).unwrap();
          let peek_col_number = col_number.checked_add_signed(col_offset).unwrap();
          number_references.get(peek_row_number).unwrap_or(&EMPTY).get(peek_col_number)
        })
        .into_iter()
        .filter(|item| if let Some(Some(_)) = item { true } else { false })
        .map(|item| item.unwrap().unwrap())
        .collect::<std::collections::HashSet<usize>>().into_iter().collect::<Vec<usize>>()
    })
    .flatten()
    .collect();
  
  selected_number_references
    .iter()
    .map(|number_reference| numbers[*number_reference])
    .fold(0, |sum, number| sum + number)
}

fn day_3_2(path: &str) -> u32 {
  let CollectParts { parts, numbers, number_references } = collect_parts(path);

  parts
    .iter()
    .filter(|(_, _, c)| *c == '*')
    .map(|(row_number, col_number, c)| {
      static EMPTY: Vec<Option<usize>> = vec![];
      static OFFSETS: [(isize, isize); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
      let part_number_references = OFFSETS
        .map(|(row_offset, col_offset)| {
          let peek_row_number = row_number.checked_add_signed(row_offset).unwrap();
          let peek_col_number = col_number.checked_add_signed(col_offset).unwrap();
          number_references.get(peek_row_number).unwrap_or(&EMPTY).get(peek_col_number)
        })
        .into_iter()
        .filter(|item| if let Some(Some(_)) = item { true } else { false })
        .map(|item| item.unwrap().unwrap())
        .collect::<std::collections::HashSet<usize>>().into_iter().collect::<Vec<usize>>();
      if part_number_references.len() == 2 {
        numbers[part_number_references[0]] * numbers[part_number_references[1]]
      } else {
        0
      }
    })
    .fold(0, |sum, number| sum + number)
}

fn main() {
  println!("1: {}", day_3_1("input"));
  println!("2: {}", day_3_2("input"));
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn day_3_1_handles_test_input() {
    assert_eq!(day_3_1("test_input"), 4361);
  }

  #[test]
  fn day_3_2_handles_test_input() {
    assert_eq!(day_3_2("test_input"), 467835);
  }
}
