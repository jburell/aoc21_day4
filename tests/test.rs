use std::{fs, rc::Rc, cell::RefCell};
use aoc21_day4::*;

fn get_data(filename: &str) -> (Vec<u32>, Vec<Rc<RefCell<Board>>>) {
  let file = fs::read_to_string(filename).unwrap();
  let mut lines = file.lines();

  let input: Vec<u32> = 
    lines
    .next()
    .map(|line| line.split(",").collect())
    .map(|cells: Vec<&str>| 
      cells.into_iter()
      .map(|cell| cell.parse().unwrap()).collect())
    .unwrap();

  let lines: Vec<&str> = 
    lines
    .map(|s| s.trim())
    .filter(|l| !l.is_empty())
    .collect();
  let board_rows = lines.chunks(SIDE_LEN_OF_BOARD);
  let boards: Vec<Rc<RefCell<Board>>> = 
    board_rows
    .map(|rows| {
      let board_data: Vec<Vec<u32>> = 
        rows.iter()
        .map(|row| {
          let vals: Vec<&str> = row.split_whitespace().collect();
          let row: Vec<u32> = 
            vals.into_iter().map(|cell| cell.parse().unwrap()).collect();
          row
        }).collect();
      board_data
    })
    .map(|data| Board::try_from(data).unwrap())
    .map(|data| RefCell::new(data))
    .map(|data| Rc::new(data))
    .collect();
  (input, boards)
}

#[test]
fn part_1() {
  // Arranage
  let data = get_data("input_part1.txt");
  // Act
  let actual = calc_sum_of_uncalled_numbers_times_winning_number((data.0, Rc::new(data.1)));
  // Assert
  match actual {
    Winner::Winner(w) => assert_eq!(51034, w),
    Winner::NoWinner => assert!(false),
  }
}
