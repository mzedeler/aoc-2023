use std::{fs};

const ERROR_MESSAGE: &str = "Some weird error happened. Help!";

fn day_1(path: &str) -> i32 {
  let file = fs::read_to_string(path).expect("ok");
  println!("{}", file);
  return 1;
}

fn main() {
  let r = day_1("input");
  println!("{}", r);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    assert_eq!(day_1("test_input"), 142);
  }
}
