
use std::fmt::Display;

use super::{super::sudoku::{
  Collapsed,
  Constraint,
  Note,
  Rule,
  Sudoku,
  FLAG_NONE,
}, generator::{GGHorizontal, GGVertical}, GGBlock};

#[derive(Clone)]
pub struct NoDuplicate {
  neighbours: Vec<usize>,
}

pub struct NoDuplicateGenerator {
  pub width: usize,
  pub height: usize,
  constraints: Vec<NoDuplicate>,
}

pub trait GroupGenerator {
  fn get_board(&self) -> (usize, usize);
  fn get_size(&self) -> (usize, usize);
  fn get_member(&self, group: usize, member: usize) -> usize;
}

impl NoDuplicate {
  fn new() -> NoDuplicate {
    NoDuplicate {
      neighbours: vec![],
    }
  }

  pub fn add(&mut self, index: usize) {
    if !self.neighbours.contains(&index) {
      self.neighbours.push(index)
    }
  }
}

impl NoDuplicateGenerator {
  pub fn new<T: Note>(rule: &Rule<T>) -> NoDuplicateGenerator {
    NoDuplicateGenerator {
      width: rule.width,
      height: rule.height,
      constraints: vec![NoDuplicate::new(); rule.size],
    }
  }

  pub fn apply<'a, T: Note>(&'a self, dst: &mut Rule<'a, T>) {
    for (i, constr) in self.constraints.iter().enumerate() {
      dst.add_constraint(i, constr);
    }
  }

  pub fn add_group(&mut self, group: &Vec<usize>) {
    for a in group.iter() {
      for b in group.iter() {
        if a != b {
          self.constraints[*a].add(*b);
          self.constraints[*b].add(*a);
        }
      }
    }
  }

  pub fn add_group_generator(&mut self, gen: &dyn GroupGenerator) {
    let (group_count, member_count) = gen.get_size();
    let mut group = vec![];
    for group_index in 0 .. group_count {
      for member in 0 .. member_count {
        group.push(gen.get_member(group_index, member));
      }
      self.add_group(&group);
      group.clear();
    }
  }

  pub fn add_vertical_group(&mut self, width: usize, height: usize, dx: usize, dy: usize) {
    self.add_group_generator(&GGVertical::new(&self, width, height, dx, dy))
  }

  pub fn add_horizontal_group(&mut self, width: usize, height: usize, dx: usize, dy: usize) {
    self.add_group_generator(&GGHorizontal::new(&self, width, height, dx, dy))
  }

  pub fn add_block_group(&mut self, width: usize, height: usize, block_width: usize, block_height: usize, dx: usize, dy: usize) {
    self.add_group_generator(&GGBlock::new(&self, width, height, block_width, block_height, dx, dy))
  }

  pub fn add_standard_group(&mut self, width: usize, height: usize, block_width: usize, block_height: usize, dx: usize, dy: usize) {
    self.add_vertical_group(width * block_width, height * block_height, dx, dy);
    self.add_horizontal_group(width * block_width, height * block_height, dx, dy);
    self.add_block_group(width, height, block_width, block_height, dx, dy);
  }

}

impl<T: Note> Constraint<T> for NoDuplicate {

  fn added(&self, _sudoku: &mut Sudoku<T>, _index: usize) -> () {
  }

  fn collapsed(&self, sudoku: &mut Sudoku<T>, _index: usize, _value: usize, note: T) -> Collapsed {
    let mut result = Collapsed::Unchanged;
    let inverse = !note;
    let rule = sudoku.rule;
    for neighbour in self.neighbours.iter() {
      if sudoku.flags[*neighbour] != FLAG_NONE {
        continue;
      }
      let old_value = sudoku.board[*neighbour];
      let new_value = old_value & inverse;
      if new_value == rule.zero {
        return Collapsed::Error;
      }
      if new_value != old_value {
        result = Collapsed::Ok;
      }
      sudoku.board[*neighbour] = new_value;
    }

    result
  }

}

impl Display for dyn GroupGenerator {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let (board_width, board_height) = self.get_board();
    let board_size = board_width * board_height;
    let mut board = vec![usize::MAX; board_size];
    let (group_count, member_count) = self.get_size();
    for group in 0 .. group_count {
      for member in 0 .. member_count {
        board[self.get_member(group, member)] = group;
      }
    }

    let print_width: usize = (group_count.ilog10() + 1).try_into().unwrap();
    for y in 0 .. board_height {
      for x in 0 .. board_width {
        if x > 0 {
          write!(f, " ")?
        }
        write!(f, "{: >print_width$}", match board[y * board_width + x] {
          usize::MAX => ".".to_string(),
          v => v.to_string(),
        })?
      }
      writeln!(f)?
    }

    Ok(())
  }
}
