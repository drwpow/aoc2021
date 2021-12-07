use std::fs;
use std::path::Path;

pub fn run() {
  let contents =
    fs::read_to_string(Path::new("./data/input1.txt")).expect("couldn’t locate data/input1.txt");
  let depths: Vec<i32> = contents
    .split("\n")
    .filter(|l| l.len() > 0)
    .map(|l| l.parse::<i32>().unwrap())
    .collect();

  let mut single_increased = 0;
  for n in 0..depths.len() {
    if n == 0 {
      continue;
    }
    if depths[n] > depths[n - 1] {
      single_increased += 1;
    }
  }

  println!("{}", format!("Part 1: {}", single_increased));

  let mut triple_increased = 0;
  for n in 0..depths.len() {
    if n < 3 {
      continue;
    }
    let prev = depths[n - 3] + depths[n - 2] + depths[n - 1];
    let current = depths[n - 2] + depths[n - 1] + depths[n];
    if current > prev {
      triple_increased += 1;
    }
  }

  println!("{}", format!("Part 2: {}", triple_increased));
}
