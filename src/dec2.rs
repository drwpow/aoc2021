use regex::Regex;

use crate::utils;

enum Direction {
  Forward,
  Up,
  Down,
}

struct Instruction {
  direction: Direction,
  amount: i32,
}

pub fn run() {
  let filepath = "data/input2.txt".to_string();
  let re = Regex::new(r"(\w+)\s+(\d+)").unwrap();
  let instructions: Vec<Instruction> = utils::read_file(filepath)
    .split("\n")
    .map(|l| {
      if l.len() == 0 {
        return None;
      }
      let caps = re.captures(&l).unwrap();
      let d_raw = &caps.get(1).unwrap().as_str();
      if d_raw.len() == 0 {
        return None;
      }
      let amount = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
      return match *d_raw {
        "forward" => Some(Instruction {
          direction: Direction::Forward,
          amount,
        }),
        "up" => Some(Instruction {
          direction: Direction::Up,
          amount,
        }),
        "down" => Some(Instruction {
          direction: Direction::Down,
          amount,
        }),
        _ => {
          println!("{}", format!("unknown direction \"{}\"", &d_raw));
          None
        }
      };
    })
    .filter(|i| i.is_some())
    .map(|i| i.unwrap())
    .collect();

  let mut x = 0;
  let mut y = 0;
  for i in &instructions {
    match i.direction {
      Direction::Forward => x += i.amount,
      Direction::Up => y -= i.amount,
      Direction::Down => y += i.amount,
    }
  }

  println!("Part 1: {}", x * y);

  let mut x2 = 0;
  let mut y2 = 0;
  let mut aim = 0;
  for i in &instructions {
    match i.direction {
      Direction::Forward => {
        x2 += i.amount;
        y2 += i.amount * aim;
      }
      Direction::Up => aim -= i.amount,
      Direction::Down => aim += i.amount,
    }
  }

  println!("Part 2: {}", x2 * y2);
}
