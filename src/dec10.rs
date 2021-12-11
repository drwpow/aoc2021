use crate::utils;

pub fn run() {
  let filepath = "data/input10.txt".to_owned();
  let lines: Vec<String> = utils::read_file(filepath)
    .split("\n")
    .filter(|l| l.len() > 0)
    .map(|l| l.to_string())
    .collect();

  let mut corrupted_score = 0;

  for l in &lines {
    let (error_i, _) = parse_line(l);
    let is_corrupted = error_i < l.len() - 1; // corrupted lines occur before last char
    if is_corrupted {
      let illegal_char = l.chars().nth(error_i + 1).unwrap();
      corrupted_score += match &illegal_char {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
      };
    }
  }

  println!("Part 1: {}", corrupted_score);

  let mut completed_scores: Vec<i64> = Vec::new();
  for l in &lines {
    let (error_i, mut paren_stack) = parse_line(l);
    let is_incomplete = error_i >= (l.len() - 1) && paren_stack.len() > 0;
    if is_incomplete {
      paren_stack.reverse();
      let mut score = 0;
      for c in &paren_stack {
        score *= 5;
        score += match c {
          '(' => 1,
          '[' => 2,
          '{' => 3,
          '<' => 4,
          _ => 0,
        }
      }
      completed_scores.push(score);
    }
  }

  completed_scores.sort();

  println!("Part 2: {}", completed_scores[completed_scores.len() / 2]);
}

// parse line until error is reached. return index of last valid char
// @returns (error index, paren stack)
fn parse_line(input: &String) -> (usize, Vec<char>) {
  let mut paren_stack: Vec<char> = Vec::new();

  for (i, c) in input.chars().enumerate() {
    // handle zero-case (empty paren_stack)
    if i == 0 && (c == ')' || c == ']' || c == '}' || c == '>') {
      return (i, paren_stack);
    }

    match c {
      '(' | '[' | '{' | '<' => {
        paren_stack.push(c);
      }
      ')' => {
        if paren_stack.len() == 0 || paren_stack.pop().unwrap() != '(' {
          return (i - 1, paren_stack);
        }
      }
      ']' => {
        if paren_stack.len() == 0 || paren_stack.pop().unwrap() != '[' {
          return (i - 1, paren_stack);
        }
      }
      '}' => {
        if paren_stack.len() == 0 || paren_stack.pop().unwrap() != '{' {
          return (i - 1, paren_stack);
        }
      }
      '>' => {
        if paren_stack.len() == 0 || paren_stack.pop().unwrap() != '<' {
          return (i - 1, paren_stack);
        }
      }
      _ => {}
    }
  }

  return (input.len(), paren_stack);
}
