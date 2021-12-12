use crate::utils;
use std::collections::HashMap;

pub fn run() {
  let filepath = "data/input11.txt".to_owned();

  // instructions declare 10x10 grid, so we can simply hardcode this
  let max_x = 10;
  let max_y = 10;

  let mut octopi: HashMap<(i32, i32), i32> = HashMap::new();
  utils::read_file(filepath)
    .split("\n")
    .filter(|l| l.len() > 0)
    .enumerate()
    .for_each(|(y, l)| {
      for (x, c) in l.chars().enumerate() {
        let nrg = c.to_string().parse::<i32>().unwrap();
        octopi.insert((x as i32, y as i32), nrg);
      }
    });

  let mut flashes: Vec<i32> = Vec::new();
  let mut n: usize = 0;

  loop {
    let mut bump_surrounding: Vec<(i32, i32)> = Vec::new();
    flashes.push(0);

    // add 1
    for x in 0..max_x {
      for y in 0..max_y {
        let plus_one = octopi.get(&(x, y)).unwrap() + 1;
        octopi.insert((x, y), plus_one); // allow "10" here; to be handled in next step
        if plus_one == 10 {
          octopi.insert((x, y), 0);
          flashes[n] += 1;
          bump_surrounding.push((x, y));
        }
      }
    }
    loop {
      if bump_surrounding.len() == 0 {
        break;
      }
      let next: Vec<(i32, i32)> = bump_surrounding.drain(..).collect();
      for (x, y) in next {
        for (x2, y2) in [
          (x - 1, y - 1),
          (x - 1, y),
          (x - 1, y + 1),
          (x, y - 1),
          // (x, y) skip self
          (x, y + 1),
          (x + 1, y - 1),
          (x + 1, y),
          (x + 1, y + 1),
        ] {
          if x2 < 0 || y2 < 0 || x2 >= max_x || y2 >= max_y {
            continue;
          }
          let energy = *octopi.get(&(x2, y2)).unwrap();
          // if octopus just flashed, ignore
          match energy {
            0 => {
              continue;
            }
            9 => {
              octopi.insert((x2, y2), 0);
              flashes[n] += 1;
              bump_surrounding.push((x2, y2));
            }
            _ => {
              octopi.insert((x2, y2), energy + 1);
            }
          }
        }
      }
    }

    if flashes[n] == 100 {
      break;
    }

    n += 1;
  }

  let mut sum_100 = 0;
  for n in 0..100 {
    sum_100 += flashes[n];
  }

  println!("Part 1: {}", sum_100);

  println!("Part 2: {}", flashes.len());
}
