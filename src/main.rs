

use std::{env::{self, Args}, error, io, usize};
use sudoku::{Note, Rule, Sudoku};
use constraint::NoDuplicateGenerator;
use rand::{thread_rng, seq::SliceRandom};

mod sudoku;
mod constraint;

fn solve<T: Note>(rule: &Rule<T>) {
  let mut s = Sudoku::new(rule);

  let mut stdin = io::stdin();
  if let Err(e) = s.read_from(&mut stdin) {
    println!("{}", e);
    return;
  }

  let mut solutions = vec![];
  s.solve(&mut solutions, 2);

  println!("{} Solution found", solutions.len());
  for (i, solution) in solutions.iter().enumerate() {
    if i > 0 {
      println!("-------------------")
    }
    println!("{}", solution);
  }
}

fn generate<T: Note>(rule: &Rule<T>, remove_amount: f32) {
  let mut s = Sudoku::new(rule);

  {
    let mut solutions = vec![];
    s.solve_random(&mut solutions, 1);
    s = solutions.pop().unwrap();
    s.make_fixed();
  }

  let mut list: Vec<usize> = (0 .. rule.size).collect();
  list.shuffle(&mut thread_rng());

  let end = (rule.size as f32 * remove_amount / 100.0).floor().abs() as usize;
  for _ in 0 .. end {
    let index = match list.pop() {
      Some(v) => v,
      None => break,
    };
    let mut copy = s.clone();
    copy.unfixed(index);
    let solution_count = copy.count_solution(2);
    if solution_count == 1 {
      s.unfixed(index);
    } 
  }
  println!("{}", s);
}

fn get_args(args: &mut Args) -> Result<(usize, usize, f32), Box<dyn error::Error>> {
  Ok((match args.next() {
    Some(v) => v.parse::<usize>()?,
    None => 3,
  }, match args.next() {
    Some(v) => v.parse::<usize>()?,
    None => 3,
  }, match args.next() {
    Some(v) => v.parse::<f32>()?,
    None => 100.0,
  }))
}

fn main() {
  let mut args = env::args();
  let name = args.next().unwrap();

  if let Some(cmd) = args.next() {
    let (width, height, remove_amount) = match get_args(&mut args) {
      Ok(v) => v,
      Err(err) => {
        println!("{}", err);
        return;
      }
    };
    let size = width * height;
    let mut rule = Rule::<u16>::new(size, size, size.try_into().unwrap(), 1);
    rule.set_grid(width, height);
    let mut nodup = NoDuplicateGenerator::new(&mut rule);
    nodup.add_standard_group(height, width, width, height, 0, 0);
    nodup.apply(&mut rule);
  
    if cmd == "solve" {
      solve(&rule);
      return;
    } else if cmd == "gen" {
      generate(&rule, remove_amount);
      return;
    }
  }

  println!("Usage:");
  println!("  {} solve [width] [height]", name);
  println!("  {} gen [width] [height] [remove amount]", name);
  println!("Where:");
  println!("  width        : Block height of the sudoku (default=3)");
  println!("  height       : Block width of the sudoku (default=3)");
  println!("  remove amount: Try to remove this much from the sudoku (default=100)");
}
