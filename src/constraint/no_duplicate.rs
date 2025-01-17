
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
pub struct NoDuplicate {
  neighbours: Vec<usize>,
}


impl NoDuplicate {
  pub fn new() -> NoDuplicate {
    NoDuplicate {
      neighbours: vec![],
    }
  }
}

impl<N: Note> ConstraintList<N> for NoDuplicate {
  fn add(&mut self, index: usize) {
    self.neighbours.push(index)
  }

  fn contain(&self, index: usize) -> bool {
    self.neighbours.contains(&index)
  }
}

impl<N: Note> Constraint<N> for NoDuplicate {
  fn added(&self, _sudoku: &mut Sudoku<N>, _index: usize) -> () {
  }

  fn collapsed(&self, sudoku: &mut Sudoku<N>, _index: usize, _value: usize, note: N) -> Collapsed {
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
