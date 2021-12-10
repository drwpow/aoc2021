use crate::utils;

struct Entry {
  // signals: Vec<String>,
  output: Vec<String>,
  mappings: [String; 10],
}

pub fn run() {
  let filename = "data/input8.txt".to_owned();
  let raw_lines: Vec<String> = utils::read_file(filename)
    .split("\n")
    .filter(|l| l.len() > 0)
    .map(|l| l.to_string())
    .collect();

  let mut count_1478 = 0;
  for l in &raw_lines {
    let entry = parse_line(l);
    // bah this is dumb but IDK how to match an array
    for o in &entry.output {
      if &entry.mappings[1] == o
        || &entry.mappings[4] == o
        || &entry.mappings[7] == o
        || &entry.mappings[8] == o
      {
        count_1478 += 1;
      }
    }
  }

  println!("Part 1: {}", count_1478);

  let mut output_sum = 0;
  for l in &raw_lines {
    let mut output_str = "".to_owned();
    let entry = parse_line(l);
    for o in &entry.output {
      for n in 0..10 {
        if &entry.mappings[n] == o {
          output_str.push_str(&*n.to_string());
          break;
        }
      }
    }
    let output = output_str.parse::<i32>().unwrap();
    output_sum += output;
  }

  println!("Part 2: {}", output_sum);
}

// turn raw line of data into signals & outputs
fn parse_line(raw_signal: &String) -> Entry {
  let parts: Vec<String> = raw_signal
    .split("|")
    .map(|l| l.trim().to_string())
    .collect();

  let mut mappings: [String; 10] = [
    "".to_owned(),
    "".to_owned(),
    "".to_owned(),
    "".to_owned(),
    "".to_owned(),
    "".to_owned(),
    "".to_owned(),
    "".to_owned(),
    "".to_owned(),
    "".to_owned(),
  ];

  let raw_signals: Vec<String> = parts[0]
    .split(" ")
    .map(|s| alpha5e(&s.to_string()))
    .collect();
  let mut signals: Vec<String> = Vec::new();
  // 1st pass: figure out 1, 4, 7, 8; collect input signals
  for s in &raw_signals {
    match s.len() {
      2 => {
        mappings[1] = String::from(s);
      }
      3 => {
        mappings[7] = String::from(s);
      }
      4 => {
        mappings[4] = String::from(s);
      }
      // 5 => 2, 3, or 5
      // 6 => 0, 6, or 9
      7 => {
        mappings[8] = String::from(s);
      }
      _ => {}
    }
    signals.push(s.to_string());
  }

  // 2nd pass: figure out 0, 2, 3, 5, 6, 9 (from 1, 4, 7, 8)
  for s in &raw_signals {
    // skip 1, 4, 7, 8 again
    if s.len() != 5 && s.len() != 6 {
      continue;
    }

    // might be a better way to do this, but it is simpler to figure out certain digits by process of elimination, hence the “weird” order

    // 3 (only 5-length code that contains all of `1`)
    if s.len() == 5
      && s.contains(mappings[1].chars().nth(0).unwrap())
      && s.contains(mappings[1].chars().nth(1).unwrap())
    {
      mappings[3] = String::from(s);
      continue;
    }

    // 6 (only 6-length code that doesn’t contain all of `1`)
    if s.len() == 6
      && (!s.contains(mappings[1].chars().nth(0).unwrap())
        || !s.contains(mappings[1].chars().nth(1).unwrap()))
    {
      mappings[6] = String::from(s);
      continue;
    }

    // 9 (only 6-lenth code that contains all of `4`)
    if s.len() == 6
      && s.contains(mappings[4].chars().nth(0).unwrap())
      && s.contains(mappings[4].chars().nth(1).unwrap())
      && s.contains(mappings[4].chars().nth(2).unwrap())
      && s.contains(mappings[4].chars().nth(3).unwrap())
    {
      mappings[9] = String::from(s);
      continue;
    }

    // 0 (only 6-length code left)
    if s.len() == 6 {
      mappings[0] = String::from(s);
      continue;
    }

    // only 5-lengths remaining…

    // 2 (only contains 2 parts of `4`)
    let mut common_4 = 0;
    for p in mappings[4].chars() {
      if s.contains(p) {
        common_4 += 1;
      }
    }
    if common_4 == 2 {
      mappings[2] = String::from(s);
      continue;
    }
    // 5 (only one left)
    else {
      mappings[5] = String::from(s);
    }
  }

  let raw_outputs: Vec<String> = parts[1]
    .split(" ")
    .map(|s| alpha5e(&s.to_string()))
    .collect();
  let mut output: Vec<String> = Vec::new();
  for o in &raw_outputs {
    output.push(o.to_string());
  }

  return Entry {
    // signals,
    output,
    mappings,
  };
}

// alphabetize string by chars
fn alpha5e(input: &String) -> String {
  let mut sorted: Vec<String> = Vec::with_capacity(input.len());
  for c in input.chars() {
    sorted.push(c.to_string());
  }
  sorted.sort();
  return sorted.join("");
}
