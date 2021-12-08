use crate::utils;

pub fn run() {
  let filepath = "data/input1.txt".to_string();
  let depths: Vec<i32> = utils::read_file(filepath)
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
