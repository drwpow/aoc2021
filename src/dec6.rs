use crate::utils;

pub fn run() {
  let filepath = "data/input6.txt".to_owned();
  let fish: Vec<u8> = utils::read_file(filepath)
    .split(",")
    .filter(|f| f.len() > 0)
    .map(|f| f.trim().parse::<u8>().unwrap())
    .collect();

  let mut counts: [u128; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 0];
  for f in &fish {
    counts[*f as usize] += 1;
  }

  for _ in 0..80 {
    let next: [u128; 9] = [
      counts[1],
      counts[2],
      counts[3],
      counts[4],
      counts[5],
      counts[6],
      counts[7] + counts[0],
      counts[8],
      counts[0],
    ];
    counts = next;
  }

  let mut day_80_count = 0;
  for c in &counts {
    day_80_count += c;
  }

  println!("Part 1: {}", day_80_count);

  for _ in 0..(256 - 80) {
    let next: [u128; 9] = [
      counts[1],
      counts[2],
      counts[3],
      counts[4],
      counts[5],
      counts[6],
      counts[7] + counts[0],
      counts[8],
      counts[0],
    ];
    counts = next;
  }

  let mut day_256_count = 0;
  for c in &counts {
    day_256_count += c;
  }

  println!("Part 2: {}", day_256_count);
}
