use crate::utils;

pub fn run() {
  let filepath = "data/input3.txt".to_string();
  let lines: Vec<String> = utils::read_file(filepath)
    .split("\n")
    .filter(|&l| l.len() > 0)
    .map(|l| l.to_string())
    .collect();

  let mut counts: [[u32; 2]; 12] = [
    [0, 0],
    [0, 0],
    [0, 0],
    [0, 0],
    [0, 0],
    [0, 0],
    [0, 0],
    [0, 0],
    [0, 0],
    [0, 0],
    [0, 0],
    [0, 0],
  ];

  for l in &lines {
    for (i, c) in l.chars().enumerate() {
      match c {
        '0' => counts[i][0] += 1,
        '1' => counts[i][1] += 1,
        _ => (),
      }
    }
  }

  let mut γ_str = "".to_owned();
  let mut ε_str = "".to_owned();

  for count in &counts {
    γ_str.push_str(if count[0] >= count[1] { "0" } else { "1" });
    ε_str.push_str(if count[0] <= count[1] { "0" } else { "1" });
  }

  let γ = usize::from_str_radix(&γ_str, 2).unwrap();
  let ε = usize::from_str_radix(&ε_str, 2).unwrap();

  println!("{}", format!("Part 1: {}", γ * ε));

  let mut oxygen_ratings = lines.clone();
  let mut oxygen_i = 0;
  loop {
    let mut zero_count: u32 = 0;
    let mut one_count: u32 = 0;
    for val in &oxygen_ratings {
      match val.chars().nth(oxygen_i).unwrap() {
        '0' => zero_count += 1,
        '1' => one_count += 1,
        _ => (),
      }
    }
    oxygen_ratings.retain(|l| {
      let c = l.chars().nth(oxygen_i).unwrap();
      if zero_count > one_count {
        return c == '0';
      } else if one_count > zero_count {
        return c == '1';
      } else {
        return c == '1'; // if equal, assert '1'
      }
    });
    if oxygen_i >= 11 || oxygen_ratings.len() == 1 {
      break;
    }
    oxygen_i += 1;
  }

  let mut co2_ratings = lines.clone();
  let mut co2_i = 0;
  loop {
    let mut zero_count: u32 = 0;
    let mut one_count: u32 = 0;
    for val in &co2_ratings {
      match val.chars().nth(co2_i).unwrap() {
        '0' => zero_count += 1,
        '1' => one_count += 1,
        _ => (),
      }
    }
    co2_ratings.retain(|l| {
      let c = l.chars().nth(co2_i).unwrap();
      if zero_count < one_count {
        return c == '0';
      } else if one_count < zero_count {
        return c == '1';
      } else {
        return c == '0'; // if equal, assert '0'
      }
    });
    if co2_i >= 11 || co2_ratings.len() == 1 {
      break;
    }
    co2_i += 1;
  }

  let oxygen_rating = usize::from_str_radix(&oxygen_ratings[0], 2).unwrap();
  let co2_rating = usize::from_str_radix(&co2_ratings[0], 2).unwrap();

  println!("{}", format!("Part 2: {}", oxygen_rating * co2_rating));
}
