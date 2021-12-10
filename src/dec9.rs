use crate::utils;
use std::collections::HashSet;

struct Point {
  x: i32,
  y: i32,
  h: i32,
}

pub fn run() {
  let filepath = "data/input9.txt".to_owned();
  let coords: Vec<Vec<i32>> = utils::read_file(filepath)
    .split("\n")
    .filter(|l| l.len() > 0)
    .map(|l| {
      let mut y_coords: Vec<i32> = Vec::new();
      for c in l.chars() {
        y_coords.push(c.to_digit(10).unwrap() as i32);
      }
      return y_coords;
    })
    .collect();
  let max_x = coords.len() as i32 - 1;
  let max_y = coords.len() as i32 - 1;

  let mut low_points: Vec<Point> = Vec::new();
  let mut low_risk_level = 0;

  for (x, y_coords) in coords.iter().enumerate() {
    for (y, h) in y_coords.iter().enumerate() {
      // bigger than left
      if (x > 0 && h >= &coords[x - 1][y]) ||
      // bigger than top
       (y > 0 && h >= &coords[x][y - 1]) ||
      // bigger than right
       ((x as i32) < max_x && h >= &coords[x + 1][y]) ||
       // bigger than bottom
       ((y as i32) < max_y && h >= &coords[x][y + 1])
      {
        continue;
      }
      low_points.push(Point {
        x: x as i32,
        y: y as i32,
        h: *h,
      });
      low_risk_level += h + 1;
    }
  }

  println!("Part 1: {}", &low_risk_level);

  let mut basins: Vec<HashSet<(i32, i32)>> = Vec::new();
  for (i, p) in low_points.iter().enumerate() {
    basins.push(HashSet::new());
    let mut to_scan: Vec<Point> = vec![Point {
      x: p.x,
      y: p.y,
      h: p.h,
    }];
    loop {
      // finished! exit
      if to_scan.len() == 0 {
        break;
      }

      let next: Vec<Point> = to_scan.drain(0..1).collect();
      let x = next[0].x;
      let y = next[0].y;
      let h = next[0].h;
      if h == 9 || basins[i].contains(&(x, y)) {
        continue;
      }
      basins[i].insert((x, y));

      // scan left, right, up, and down
      let walk: Vec<(i32, i32)> = vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
      for (x2, y2) in &walk {
        // skip bad coords
        if *x2 < 0 || *y2 < 0 || *x2 > max_x || *y2 > max_y {
          continue;
        }
        // skip already-scanned
        if basins[i].contains(&(*x2, *y2)) {
          continue;
        }
        // skip 9 and smaller/same numbers
        let h2 = coords[*x2 as usize][*y2 as usize];
        if h2 == 9 || h2 <= h {
          continue;
        }
        to_scan.push(Point {
          x: *x2,
          y: *y2,
          h: h2,
        });
      }
    }
  }

  let mut basin_sizes: Vec<usize> = Vec::new();
  for b in &basins {
    basin_sizes.push(b.len());
  }
  basin_sizes.sort();
  basin_sizes.reverse();

  println!(
    "Part 1: {}",
    basin_sizes[0] * basin_sizes[1] * basin_sizes[2]
  );
}
