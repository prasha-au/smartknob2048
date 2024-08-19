#![cfg_attr(not(test), no_std)]

use rand::{rngs::SmallRng, RngCore, SeedableRng};

type BoardLine = [u16; 4];
type Board = [BoardLine; 4];

fn squash_line(line: &mut BoardLine) {
  let mut idx = 0;
  let find_next_value_index = |line: &BoardLine, from_idx: usize| (from_idx..4).find(|&i| line[i] != 0);
  while let Some(next_value_idx) = find_next_value_index(line, idx + 1) {
    if line[idx] == 0 {
      line[idx] = line[next_value_idx];
      line[next_value_idx] = 0;
      continue;
    }

    if line[idx] == line[next_value_idx] {
      line[idx] *= 2;
      line[next_value_idx] = 0;
    }

    idx += 1;
  }
}

fn add_random_tile(board: &mut Board, seed: u64) {
  let mut small_rng = SmallRng::seed_from_u64(seed);
  loop {
    let i = small_rng.next_u32() as usize % 4;
    let j = small_rng.next_u32() as usize % 4;
    if board[i][j] == 0 {
      board[i][j] = 2;
      return;
    }
  }
}

pub struct Game {
  pub board: Board,
  rng_seed: u64,
}

impl Game {
  pub fn create() -> Self {
    Self {
      board: [[0; 4]; 4],
      rng_seed: 0,
    }
  }

  pub fn add_tile(&mut self) {
    self.rng_seed += self.board.iter().flatten().sum::<u16>() as u64;
    add_random_tile(&mut self.board, self.rng_seed);
  }

  pub fn move_left(&mut self) {
    for i in 0..4 {
      let mut line = self.board[i];
      squash_line(&mut line);
      self.board[i] = line;
    }
  }

  pub fn move_right(&mut self) {
    for i in 0..4 {
      let mut line = self.board[i];

      line.reverse();
      squash_line(&mut line);
      line.reverse();

      self.board[i] = line;
    }
  }

  pub fn rotate(&mut self) {
    let mut new_board = [[0; 4]; 4];
    for i in 0..4 {
      for (j, v) in self.board[i].iter().enumerate() {
        new_board[j][3 - i] = *v;
      }
    }
    self.board = new_board;
  }

  pub fn check_win_loss(&self) -> Option<bool> {
    let mut has_empty = false;
    for i in 0..4 {
      for v in self.board[i] {
        if v == 2048 {
          return Some(true);
        }
        if v == 0 {
          has_empty = true;
        }
      }
    }
    if has_empty {
      None
    } else {
      Some(false)
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_squash_line_basic() {
    let mut line = [2, 2, 0, 0];
    squash_line(&mut line);
    assert_eq!(line, [4, 0, 0, 0]);
  }

  #[test]
  fn test_squash_line_moves_zeros_to_end() {
    let mut line = [0, 0, 2, 0];
    squash_line(&mut line);
    assert_eq!(line, [2, 0, 0, 0]);
  }

  #[test]
  fn test_squash_line_dual() {
    let mut line = [2, 2, 2, 2];
    squash_line(&mut line);
    assert_eq!(line, [4, 4, 0, 0]);
  }

  #[test]
  fn test_squash_line_picks_correct_value() {
    let mut line = [2, 2, 2, 4];
    squash_line(&mut line);
    assert_eq!(line, [4, 2, 4, 0]);
  }

  #[test]
  fn test_squash_line_different_values() {
    let mut line = [2, 2, 4, 4];
    squash_line(&mut line);
    assert_eq!(line, [4, 8, 0, 0]);
  }

  #[test]
  fn test_squash_line_does_not_merge_different_values() {
    let mut line = [2, 4, 8, 16];
    squash_line(&mut line);
    assert_eq!(line, [2, 4, 8, 16]);
  }

  #[test]
  fn test_add_random_tile() {
    let mut board = [[0; 4]; 4];
    add_random_tile(&mut board, 54);
    let total_of_tiles = board.iter().flatten().sum::<u16>();
    assert_eq!(total_of_tiles, 2);
  }

  #[test]
  fn test_add_random_tile_with_different_seeds() {
    let mut board1 = [[0; 4]; 4];
    add_random_tile(&mut board1, 10);
    let mut board2 = [[0; 4]; 4];
    add_random_tile(&mut board2, 12);
    assert_ne!(board1, board2);
  }

  #[test]
  fn test_add_random_tile_does_not_use_same_location() {
    let mut board = [[0; 4]; 4];
    add_random_tile(&mut board, 0);
    add_random_tile(&mut board, 0);
    let total_of_tiles = board.iter().flatten().sum::<u16>();
    assert_eq!(total_of_tiles, 4);
  }

  #[test]
  fn test_initialization() {
    let game = Game::create();
    assert_eq!(game.board, [[0; 4]; 4]);
  }

  #[test]
  fn test_add_tile() {
    let mut game = Game::create();
    game.add_tile();
    let total_of_tiles = game.board.iter().flatten().sum::<u16>();
    assert_eq!(total_of_tiles, 2);
    game.add_tile();
    let total_of_tiles = game.board.iter().flatten().sum::<u16>();
    assert_eq!(total_of_tiles, 4);
  }

  #[test]
  fn test_move_left() {
    let mut game = Game::create();
    game.board = [[0, 2, 0, 2], [0, 0, 4, 2], [8, 0, 0, 2], [2, 2, 0, 2]];
    game.move_left();
    assert_eq!(game.board, [[4, 0, 0, 0], [4, 2, 0, 0], [8, 2, 0, 0], [4, 2, 0, 0],]);
  }

  #[test]
  fn test_move_right() {
    let mut game = Game::create();
    game.board = [[0, 2, 0, 2], [4, 2, 0, 0], [8, 0, 0, 2], [2, 2, 0, 2]];
    game.move_right();
    assert_eq!(game.board, [[0, 0, 0, 4], [0, 0, 4, 2], [0, 0, 8, 2], [0, 0, 2, 4],]);
  }

  #[test]
  fn test_rotate() {
    let mut game = Game::create();
    game.board = [[2, 4, 0, 8], [4, 2, 4, 0], [0, 4, 2, 4], [4, 0, 4, 2]];
    game.rotate();
    assert_eq!(game.board, [[4, 0, 4, 2], [0, 4, 2, 4], [4, 2, 4, 0], [2, 4, 0, 8],]);
    game.rotate();
    assert_eq!(game.board, [[2, 4, 0, 4], [4, 2, 4, 0], [0, 4, 2, 4], [8, 0, 4, 2],]);
  }

  #[test]
  fn test_check_win_loss_for_winning_game() {
    let mut game = Game::create();
    game.board = [[0, 2, 0, 0], [0, 8, 0, 0], [0, 0, 16, 0], [4, 0, 0, 2048]];
    assert_eq!(game.check_win_loss(), Some(true));
  }

  #[test]
  fn test_check_win_loss_for_losing_game() {
    let mut game = Game::create();
    game.board = [[2, 4, 2, 4], [4, 2, 4, 2], [2, 4, 2, 4], [4, 2, 4, 2]];
    assert_eq!(game.check_win_loss(), Some(false));
  }

  #[test]
  fn test_check_win_loss_for_game_in_progress() {
    let mut game = Game::create();
    game.board = [[2, 2, 2, 2], [2, 2, 0, 2], [2, 2, 2, 2], [2, 2, 2, 2]];
    assert_eq!(game.check_win_loss(), None);
  }
}
