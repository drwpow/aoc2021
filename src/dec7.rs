use crate::utils;

pub fn run() {
  let filename = "data/input7.txt".to_owned();
  let mut crabs: Vec<i32> = utils::read_file(filename)
    .split(",")
    .filter(|l| l.len() > 0)
    .map(|l| l.trim().parse::<i32>().unwrap())
    .collect();
  crabs.sort();

  let median: i32 = crabs.len() as i32 / 2; // note: this will be rounded!
  let mut fuel = i32::MAX;
  for n1 in 0..median {
    let n = median - (median / 2) + n1;
    let next = calc_fuel(&crabs, &n);
    if next < fuel {
      fuel = next;
    }
  }

  println!("Part 1: {}", &fuel);

  let mut fuel_2 = f64::MAX;
  for n1 in 0..median {
    let n = median - (median / 2) + n1;
    let next = calc_fuel_exp(&crabs, &n);
    if next < fuel_2 {
      fuel_2 = next;
    }
  }

  println!("Part 2: {}", &fuel_2);
}

fn calc_fuel(positions: &Vec<i32>, point: &i32) -> i32 {
  let mut fuel = 0;
  for p in positions {
    fuel += (point - p).abs();
  }
  return fuel;
}

fn calc_fuel_exp(positions: &Vec<i32>, point: &i32) -> f64 {
  let mut fuel = 0.0;
  for p in positions {
    let diff = f64::from(point - p).abs();
    let move_cost = 0.5 * diff * (diff + 1.0); // move_cost = n plus sum of all integers below n (yes I googled for this)
    fuel += move_cost;
  }
  return fuel;
}
