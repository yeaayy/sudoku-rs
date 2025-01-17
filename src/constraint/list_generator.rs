#![allow(dead_code)]

use std::{fmt::Display, marker::PhantomData};

use super::{super::sudoku::{
  Constraint,
  Note,
  Rule,
}, generator::{
  GGBlock,
  GGHorizontal,
  GGVertical,
}};

pub trait GroupGenerator {
  fn get_board(&self) -> (usize, usize);
  fn get_size(&self) -> (usize, usize);
  fn get_member(&self, group: usize, member: usize) -> usize;
}

pub trait ConstraintList<N>: Constraint<N> + Clone {
  fn add(&mut self, index: usize);
  fn contain(&self, index: usize) -> bool;

  fn add_unique(&mut self, index: usize) {
    if !self.contain(index) {
      self.add(index)
    }
  }
}

pub struct ConstraintListGenerator<T, N>
where
  T: ConstraintList<N>,
  N: Note,
{
  pub width: usize,
  pub height: usize,
  constraints: Vec<T>,
  phantom: PhantomData<N>
}

impl<T, N> ConstraintListGenerator<T, N>
where
  T: ConstraintList<N>,
  N: Note
{
  pub fn new(one: T, rule: &Rule<N>) -> ConstraintListGenerator<T, N> {
    ConstraintListGenerator::<T, N> {
      width: rule.width,
      height: rule.height,
      constraints: vec![one; rule.size],
      phantom: PhantomData,
    }
  }

  pub fn apply<'a>(&'a self, dst: &mut Rule<'a, N>) {
    for (i, constr) in self.constraints.iter().enumerate() {
      dst.add_constraint(i, constr);
    }
  }

  pub fn add_group(&mut self, group: &Vec<usize>) {
    for &a in group.iter() {
      for &b in group.iter() {
        if a != b {
          self.constraints[a].add_unique(b);
          self.constraints[b].add_unique(a);
        }
      }
    }
  }

  pub fn add_every(&mut self, func: fn (width: usize, height: usize, i: usize) -> Vec<usize>) {
    for i in 0 .. (self.width * self.height) {
      let list = func(self.width, self.height, i);
      for j in list {
        self.constraints[i].add_unique(j)
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

#[inline]
pub fn get_cell(width: usize, height: usize, i: usize, dx: i32, dy: i32) -> Option<usize> {
  let mut x = i % width;
  let mut y = i / width;
  if dx < 0 {
    if (-dx) as usize > x {
      return None
    } else {
      x -= (-dx) as usize;
    }
  } else {
    if (dx as usize + x) >= width {
      return None
    } else {
      x += dx as usize;
    }
  }
  if dy < 0 {
    if (-dy) as usize > y {
      return None
    } else {
      y -= (-dy) as usize;
    }
  } else {
    if (dy as usize + y) >= height {
      return None
    } else {
      y += dy as usize;
    }
  }
  Some(x + y * width)
}

#[inline]
pub fn g_generate_neighbour(width: usize, height: usize, i: usize, list: Vec<(i32, i32)>) -> Vec<usize> {
  let mut result = vec![];
  for (dx, dy) in list {
    if let Some(j) = get_cell(width, height, i, dx, dy) {
      result.push(j)
    }
  }
  result
}

pub fn g_adjacent(width: usize, height: usize, i: usize) -> Vec<usize> {
  g_generate_neighbour(width, height, i, vec![
    ( 0,  1),
    ( 0, -1),
    ( 1,  0),
    (-1,  0),
  ])
}

pub fn g_diagonal(width: usize, height: usize, i: usize) -> Vec<usize> {
  g_generate_neighbour(width, height, i, vec![
    ( 1,  1),
    (-1,  1),
    ( 1, -1),
    (-1, -1),
  ])
}

pub fn g_king_move(width: usize, height: usize, i: usize) -> Vec<usize> {
  g_generate_neighbour(width, height, i, vec![
    ( 0,  1),
    ( 0, -1),
    ( 1,  0),
    (-1,  0),
    ( 1,  1),
    (-1,  1),
    ( 1, -1),
    (-1, -1),
  ])
}

pub fn g_horse_move(width: usize, height: usize, i: usize) -> Vec<usize> {
  g_generate_neighbour(width, height, i, vec![
    ( 1,  2), ( 2,  1),
    (-1,  2), (-2,  1),
    ( 1, -2), ( 2, -1),
    (-1, -2), (-2, -1),
  ])
}
