use crate::utils;
use regex::Regex;
use std::cmp;
use std::collections::HashMap;

struct Line {
  x1: i32,
  y1: i32,
  x2: i32,
  y2: i32,
}

pub fn run() {
  let re =
    Regex::new(r"(?P<x1>\d+)\s*,\s*(?P<y1>\d+)\s*->\s*(?P<x2>\d+)\s*,\s*(?P<y2>\d+)").unwrap(); // x1,y1 -> x2,y2

  let filename = "data/input5.txt".to_owned();
  let lines: Vec<Line> = utils::read_file(filename)
    .split("\n")
    .filter(|&l| l.len() > 0)
    .map(|l| {
      let caps = re.captures(l).unwrap();
      Line {
        x1: caps.name("x1").unwrap().as_str().parse::<i32>().unwrap(),
        y1: caps.name("y1").unwrap().as_str().parse::<i32>().unwrap(),
        x2: caps.name("x2").unwrap().as_str().parse::<i32>().unwrap(),
        y2: caps.name("y2").unwrap().as_str().parse::<i32>().unwrap(),
      }
    })
    .collect();

  let mut cardinal_grid: HashMap<i32, HashMap<i32, u32>> = HashMap::new(); // x: y: count

  for l in &lines {
    if l.x1 != l.x2 && l.y1 != l.y2 {
      continue; // for part 1 ignore diagonal lines
    }

    let length = cmp::max((l.x2 - l.x1).abs(), (l.y2 - l.y1).abs());
    for n in 0..=length {
      let x_diff = (l.x2 - l.x1) * n / length;
      let y_diff = (l.y2 - l.y1) * n / length;
      let y_map = cardinal_grid.entry(l.x1 + x_diff).or_insert(HashMap::new());
      let coord_count = y_map.entry(l.y1 + y_diff).or_insert(0);
      *coord_count += 1;
    }
  }

  let mut cardinal_overlap = 0;
  for x in cardinal_grid.keys() {
    for y in cardinal_grid[x].keys() {
      if cardinal_grid[x][y] > 1 {
        cardinal_overlap += 1;
      }
    }
  }

  println!("Part 1: {}", cardinal_overlap);

  let mut grid: HashMap<i32, HashMap<i32, u32>> = HashMap::new(); // x: y: count

  for l in &lines {
    let length = cmp::max((l.x2 - l.x1).abs(), (l.y2 - l.y1).abs()); // note: lines will either be at 0°, 45°, or 90° so we can cheat a lil

    for n in 0..=length {
      let x_diff = (l.x2 - l.x1) * n / length;
      let y_diff = (l.y2 - l.y1) * n / length;
      let y_map = grid.entry(l.x1 + x_diff).or_insert(HashMap::new());
      let coord_count = y_map.entry(l.y1 + y_diff).or_insert(0);
      *coord_count += 1;
    }
  }

  let mut overlap_points = 0;
  for x in grid.keys() {
    for y in grid[x].keys() {
      if grid[x][y] > 1 {
        overlap_points += 1;
      }
    }
  }

  println!("Part 2: {}", overlap_points);
}
