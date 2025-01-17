#![allow(dead_code)]

use std::{cmp, fmt::{self, Display, Formatter}, fs::File, io::{self, BufReader, Read}, ops::{BitAnd, BitOr, Not, Shl, Shr}};
use rand::{thread_rng, seq::SliceRandom};

pub trait Note
  : Shl<i32, Output = Self>
  + Shr<i32, Output = Self>
  + PartialEq<Self>
  + BitOr<Self, Output = Self>
  + BitAnd<Self, Output = Self>
  + Not<Output = Self>
  + Copy
{}

impl<T> Note for T
where T
  : Shl<i32, Output = Self>
  + Shr<i32, Output = Self>
  + PartialEq<Self>
  + BitOr<Self, Output = Self>
  + BitAnd<Self, Output = Self>
  + Not<Output = Self>
  + Copy
{}

pub enum Collapsed {
  Ok,
  Unchanged,
  Error,
}

pub trait Constraint<T> {
  fn added(&self, sudoku: &mut Sudoku<T>, index: usize) -> ();
  fn collapsed(&self, sudoku: &mut Sudoku<T>, index: usize, value: usize, note: T) -> Collapsed;
}

pub struct  Rule<'a, T = u32> {
  pub width: usize,
  pub height: usize,
  pub size: usize,
  x_split: usize,
  y_split: usize,
  pub zero: T,
  pub all: T,
  pub note: Vec<T>,
  constraints: Vec<Vec<&'a dyn Constraint<T>>>,
}

pub type Flag = u8;
pub const FLAG_NONE: Flag = 0;
pub const FLAG_OK: Flag = 1;
pub const FLAG_IGNORED: Flag = 2;
pub const FLAG_FIXED: Flag = 4;

#[derive(Clone)]
pub struct Sudoku<'a, T> {
  pub rule: &'a Rule<'a, T>,
  pub board: Vec<T>,
  pub flags: Vec<Flag>,
}

fn get_note_index<T>(note: &Vec<T>, value: T) -> usize
where T: cmp::PartialEq<T>
{
  for (i, x) in note.iter().enumerate() {
    if value == *x {
      return i
    }
  }
  return usize::MAX;
}

impl<'a, T: Note> Rule<'a, T> {
  pub fn new(width: usize, height: usize, max_value: u32, one: T) -> Rule<'a, T> {
    let size = width * height;
    let mut all = one;
    let mut curr = one;
    let mut note = vec![one];

    for _ in 1 .. max_value {
      curr = curr << 1;
      all = all | curr;
      note.push(curr);
    }

    Rule {
      width,
      height,
      size,
      x_split: 3,
      y_split: 3,
      // max: max_value,
      zero: (one << 1) & one,
      all,
      note,
      constraints: vec![vec![]; size]
    }
  }

  pub fn add_constraint(&mut self, index: usize, constraint: &'a dyn Constraint<T>) {
    self.constraints[index].push(constraint);
  }

  pub fn set_grid(&mut self, width: usize, height: usize) {
    self.x_split = width;
    self.y_split = height;
  }

}

impl<'s, T: Note> Sudoku<'s, T> {

  pub fn new<'a>(rule: &'a Rule<T>) -> Sudoku<'a, T> {
    let mut result = Sudoku {
      rule,
      board: vec![rule.all; rule.size],
      flags: vec![0; rule.size],
    };

    for i in 0 .. rule.size {
      for constr in &rule.constraints[i] {
        constr.added(&mut result, i)
      }
    }
    result
  }

  pub fn make_fixed(&mut self) {
    for i in 0 .. self.rule.size {
      if self.flags[i] & FLAG_OK == FLAG_OK {
        self.flags[i] |= FLAG_FIXED;
      }
    }
  }

  pub fn collapse(&mut self, index: usize, value: usize) -> Collapsed {
    let rule = self.rule;
    if self.flags[index] & FLAG_OK == FLAG_OK {
      return Collapsed::Unchanged;
    }
    let note = rule.note[value];
    self.flags[index] |= FLAG_OK;
    self.board[index] = note;

    let mut result = Collapsed::Unchanged;
    for constr in rule.constraints[index].iter() {
      match constr.collapsed(self, index, value, note) {
        Collapsed::Ok => result = Collapsed::Ok,
        Collapsed::Error => return Collapsed::Error,
        Collapsed::Unchanged => {},
      }
    }

    result
  }

  pub fn collapse_avail(&mut self) -> Option<usize> {
    let rule = self.rule;
    let mut first_unsolved ;
    'repeat: loop {
      first_unsolved = usize::MAX;
      for i in 0 .. rule.size {
        // Skip if ignored or ok.
        if self.flags[i] != FLAG_NONE {
          continue;
        }

        let note = self.board[i];
        if note == rule.zero {
          return None;
        }
        let value = get_note_index(&rule.note, note);
        if value == usize::MAX {
          first_unsolved = i;
        } else {
          match self.collapse(i, value) {
            Collapsed::Ok => continue 'repeat,
            Collapsed::Error => return None,
            Collapsed::Unchanged => {},
          }
        }
      }
      break;
    }
    return Some(first_unsolved);
  }

  pub fn count_solution(&mut self, limit: usize) -> usize {
    let rule = self.rule;
    let first_unsolved = match self.collapse_avail() {
      None => return 0,
      Some(usize::MAX) => return 1,
      Some(x) => x,
    };

    let mut note = self.board[first_unsolved];
    let mut value = 0;
    let mut result: usize = 0;
    while note != rule.zero && result < limit {
      if (note & rule.note[0]) != rule.zero {
        let mut copy = self.clone();
        match copy.collapse(first_unsolved, value) {
          Collapsed::Error => {},
          _ => result += copy.count_solution(limit),
        }
      }
      note = note >> 1;
      value += 1;
    }

    result
  }

  fn solve_random_recursive(&mut self, dst: &mut Vec<Sudoku<'s, T>>, limit: usize) -> bool {
    let rule = self.rule;
    let first_unsolved = match self.collapse_avail() {
      None => return false,
      Some(usize::MAX) => return true,
      Some(x) => x,
    };

    let mut note = self.board[first_unsolved];
    let mut value = 0;
    let mut candidates = Vec::new();
    while note != rule.zero {
      if (note & rule.note[0]) != rule.zero {
        candidates.push(value);
      }
      note = note >> 1;
      value += 1;
    }
    candidates.shuffle(&mut thread_rng());

    while dst.len() < limit {
      let mut copy = self.clone();
      match candidates.pop() {
        None => break,
        Some(value) => match copy.collapse(first_unsolved, value) {
          Collapsed::Error => {},
          _ => if Sudoku::solve_random_recursive(&mut copy, dst, limit) {
            dst.push(copy);
          },
        },
      }
    }

    return false;
  }

  pub fn solve_random(&mut self, dst: &mut Vec<Sudoku<'s, T>>, limit: usize) {
    let mut copy = self.clone();
    if copy.solve_random_recursive(dst, limit) {
      dst.push(copy);
    }
  }

  fn solve_recursive(&mut self, dst: &mut Vec<Sudoku<'s, T>>, limit: usize) -> bool {
    let rule = self.rule;
    let first_unsolved = match self.collapse_avail() {
      None => return false,
      Some(usize::MAX) => return true,
      Some(x) => x,
    };

    let mut note = self.board[first_unsolved];
    let mut value = 0;
    while note != rule.zero && dst.len() < limit {
      if (note & rule.note[0]) != rule.zero {
        let mut copy = self.clone();
        match copy.collapse(first_unsolved, value) {
          Collapsed::Error => {},
          _ => if Sudoku::solve_recursive(&mut copy, dst, limit) {
            dst.push(copy);
          }
        }
      }
      note = note >> 1;
      value += 1;
    }

    false
  }

  pub fn solve(&mut self, dst: &mut Vec<Sudoku<'s, T>>, limit: usize) {
    let mut copy = self.clone();
    if copy.solve_recursive(dst, limit) {
      dst.push(copy);
    }
  }

  pub fn ignore(&mut self, index: usize) {
    self.flags[index] |= FLAG_IGNORED;
  }

  pub fn read_from_file(&mut self, filename: &str) -> io::Result<()> {
    let mut file = File::open(filename)?;
    self.read_from(&mut file)
  }

  pub fn read_from(&mut self, src: &mut dyn Read) -> io::Result<()> {
    let mut reader = BufReader::new(src);

    let mut buffer = String::new();

    reader.read_to_string(&mut buffer)?;

    let mut i = 0;
    let mut it = buffer.chars();
    let mut buff = String::new();
    loop {
      let mut finish = false;
      let mut increment = false;
      loop  {
        match it.next() {
          Some(ch) => {
            if ch.is_ascii_digit() {
              buff.push(ch);
            } else if ch == '.' {
              increment = true;
              break;
            } else {
              break;
            }
          }
          None => {
            finish = true;
            break;
          }
        }
      }
      if buff.len() > 0 {
        let value = buff.parse::<usize>().unwrap();
        if value > 0 {
          self.collapse(i, value - 1);
          i += 1;
        }
        buff.clear();
      }
      if increment {
        i += 1;
      }

      if finish || i >= self.rule.size {
        break;
      }
    }

    Ok(())
  }

  pub fn unfixed(&mut self, index: usize) {
    let rule = self.rule;
    self.flags[index] &= !FLAG_FIXED;

    for i in 0 .. rule.size {
      self.flags[i] &= !FLAG_OK;
      if self.flags[i] & FLAG_FIXED != FLAG_FIXED {
        self.board[i] = rule.all;
      }
    }

    for i in 0 .. rule.size {
      if (self.flags[i] & FLAG_FIXED) == FLAG_FIXED {
        let value = get_note_index(&rule.note, self.board[i]);
        assert_ne!(value, usize::MAX);
        self.collapse(i, value);
      }
    }
  }

}

impl<T: Note> Display for Sudoku<'_, T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    let rule = self.rule;
    let mut i = 0;
    let space: usize = (rule.note.len().ilog10() + 1).try_into().unwrap();
    for y in 0 .. rule.height {
      for x in 0 .. rule.width {
        if x > 0 {
          write!(f, " ")?
        }
        if x % rule.x_split == 0 && x > 0 {
          write!(f, " ")?
        }
        write!(f, "{: >space$}", if (self.flags[i] & FLAG_IGNORED) == FLAG_IGNORED {
          " ".to_string()
        } else if self.flags[i] != FLAG_NONE {
          match get_note_index::<T>(&rule.note, self.board[i]) {
            usize::MAX => ".".to_string(),
            n => (n + 1).to_string()
          }
        } else {
          ".".to_string()
        })?;
        i += 1;
      }
      if y < rule.height - 1 {
        if y % rule.y_split == rule.y_split - 1  {
          writeln!(f)?
        }
        writeln!(f)?
      }
    }
    Ok(())
  }
}
