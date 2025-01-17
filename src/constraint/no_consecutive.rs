
use super::{
  ConstraintList,
  super::sudoku::{
    FLAG_NONE,
    Collapsed,
    Constraint,
    Note,
    Sudoku,
  }
};

#[derive(Clone)]
pub struct NoConsecutive {
  neighbours: Vec<usize>,
}


impl NoConsecutive {
  pub fn new() -> NoConsecutive {
    NoConsecutive {
      neighbours: vec![],
    }
  }
}

impl<N: Note> ConstraintList<N> for NoConsecutive {
  fn add(&mut self, index: usize) {
    self.neighbours.push(index)
  }

  fn contain(&self, index: usize) -> bool {
    self.neighbours.contains(&index)
  }
}

impl<N: Note> Constraint<N> for NoConsecutive {
  fn added(&self, _sudoku: &mut Sudoku<N>, _index: usize) -> () {
  }

  fn collapsed(&self, sudoku: &mut Sudoku<N>, _index: usize, _value: usize, note: N) -> Collapsed {
    let mut result = Collapsed::Unchanged;
    let rule = sudoku.rule;
    let inverse = !((note << 1 | note >> 1) & rule.all);
    for &neighbour in self.neighbours.iter() {
      if sudoku.flags[neighbour] != FLAG_NONE {
        continue;
      }
      let old_value = sudoku.board[neighbour];
      let new_value = old_value & inverse;
      if new_value == rule.zero {
        return Collapsed::Error;
      }
      if new_value != old_value {
        result = Collapsed::Ok;
      }
      sudoku.board[neighbour] = new_value;
    }

    result
  }
}
