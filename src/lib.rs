use std::{cell::RefCell, rc::Rc, rc::*};

pub const SIDE_LEN_OF_BOARD: usize = 5;
pub type IsMarked = bool;
pub type Cell = (IsMarked, u32);
#[derive(Clone)]
pub struct Board([[Cell; SIDE_LEN_OF_BOARD]; SIDE_LEN_OF_BOARD]);

impl TryFrom<Vec<Vec<u32>>> for Board {
  type Error = String;
  fn try_from(data: Vec<Vec<u32>>) -> Result<Self, Self::Error> {
    if data.len() == 5 {
        data.iter().enumerate()
        .fold(Ok([[(false, 0u32); 5]; 5]), |acc, (row, vec)| {
          match acc {
            Ok(mut acc) => {
              let with_markers: Vec<Cell> = 
                vec.iter()
                .map(|elem| (false, *elem))
                .collect();
              match with_markers.try_into() {
                Ok(d) => { acc[row] = d; Ok(acc)}
                Err(_e) => Err("Could not create board".to_string())
              }
            }
            Err(e) => Err(e)
          }
        })
        .map_err(|_e| "Could not create board".to_string())
        .map(|b_data| Board(b_data))       
    } else {
      Err(format!("Board must have {} rows", SIDE_LEN_OF_BOARD))
    }
  }
}

type BoardNum = u32;
type WinningNum = u32;
enum GameState {
  GameWon(BoardNum, WinningNum),
  Continue,
}

type Score = u32;
pub enum Winner {
  Winner(Score),
  NoWinner,
}

pub fn calc_sum_of_uncalled_numbers_times_winning_number(data: (Vec<u32>, Rc<Vec<Rc<RefCell<Board>>>>)) -> Winner {
  let numbers_to_call = data.0;
  let boards = data.1;

  let game_state = 
    numbers_to_call.iter()
    .fold(GameState::Continue, |state, called_num| {
      match state {
        GameState::Continue => 
          match play_boards(boards.clone(), *called_num) {
            GameState::GameWon(n, w) => {
              GameState::GameWon(n, w)
            }
            GameState::Continue => GameState::Continue
          }
        GameState::GameWon(n, w) => GameState::GameWon(n, w)
      }
    });

  match game_state {
    GameState::GameWon(board_num, called_num) => {
      let winning_board_data = boards[board_num as usize].borrow().0;
      let non_called_nums_sum = 
        winning_board_data.into_iter()
        .fold(0, |acc, row| {
          let row_sum: u32 = 
          row.iter()
          .filter(|(marker, _)| !marker)
          .map(|(_, v)| v)
          .sum();
          acc + row_sum
        });
      Winner::Winner(non_called_nums_sum * called_num)
    }
    GameState::Continue => Winner::NoWinner,
  }
}

fn play_boards(boards: Rc<Vec<Rc<RefCell<Board>>>>, called_num: u32) -> GameState {
  boards.iter().enumerate()
  .fold(GameState::Continue, |state, (board_num, board)| {
    match state {
      GameState::Continue => {
        match play_board(board.clone(), called_num) {
          true => GameState::GameWon(board_num as u32, called_num),
          false => GameState::Continue,
        }
      }
      GameState::GameWon(n, w) => GameState::GameWon(n, w),
    }
  })
}

fn play_board(board: Rc<RefCell<Board>>, called_num: u32) -> bool {
  for y in 0..SIDE_LEN_OF_BOARD{
    for x in 0..SIDE_LEN_OF_BOARD {
      {
        let b: &mut Board = &mut board.borrow_mut();
        if b.0[y][x].1 == called_num { 
          b.0[y][x].0 = true;
        } 
      }
      if check_win_conditions(board.borrow().clone()) {
        return true;
      }
    }
  }
  false
}

fn check_win_conditions(board: Board) -> bool {
  let mut x;
  let mut ys = [0; SIDE_LEN_OF_BOARD];
  for (pos_y, r) in board.0.iter().enumerate() {
    x = 0;
    for c in r {
      if c.0 == true { 
        x += 1;
        if x == SIDE_LEN_OF_BOARD {
          return true;
        }
        ys[pos_y] += 1;
        if ys[pos_y] == SIDE_LEN_OF_BOARD {
          return true;
        }
      } 
    }
  }
  false
}