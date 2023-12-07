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

// impl State {
//   fn next(mut self, cell: Cell) {
//     let mut next_state = match cell {
//       Cell::Empty() => Empty(),
//       Cell::Part(_) => Part(),
//       Cell::Digit(_) => ParsingSchemaNumber(),
//     };
//     match (self, next_state) {
//       (Initial() | Empty() | Part(), ParsingSchemaNumber(_)) => {

//       },
//       (ParsingSchemaNumber)
//     }
//   }
// }

// impl SchemaNumber {
//   fn is_part_no(mut self: SchemaNumber) {
//     self.is_part_no = true;
//   }

//   fn add_digit(mut self: SchemaNumber, digit: u32) {
//     self.number = self.number * 10 + digit;
//   }
// }

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

// fn has_part(schematic: &Vec<Vec<Cell>>, row_number: usize, col_number: usize) -> bool {
//   schematic[row_number][col_number] == Cell::Digit(1)
// }

fn day_3_1(path: &str) -> u32 {
  let schematic = parse_file(path);
  let mut state = State::Initial();
  let mut parts: Vec<(usize, usize)> = vec![];
  let mut numbers: Vec<Vec<(u32, u32)>> = vec![vec![(0, 0); schematic[0].len()]; schematic.len()];
  let mut number_id: u32 = 0;

  let mut store_number =  |state: &State, row_number: usize| {
    if let State::ParsingSchemaNumber(schema_number) = state {
      number_id += 1;
      for col_number in schema_number.col_start ..= schema_number.col_end {
        numbers[row_number][col_number] = (number_id, schema_number.number);
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
  println!("{:?} {:?}", numbers, parts);
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
