use crate::utils;
use regex::Regex;

pub fn run() {
  let filepath = "data/input4.txt".to_owned();
  let lines: Vec<String> = utils::read_file(filepath)
    .split("\n")
    .map(|l| l.to_string())
    .collect();

  let re = Regex::new(r"(\d+)").unwrap();

  let drawing: Vec<u32> = lines
    .get(0)
    .unwrap()
    .split(",")
    .map(|n| n.parse::<u32>().unwrap())
    .collect();

  let mut boards: Vec<[u32; 25]> = Vec::new();
  let mut squares_marked: Vec<[bool; 25]> = Vec::new();
  let mut board_i: usize = 0;
  let mut last_board: [u32; 25] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
  ];
  for (line_i, l) in lines.iter().enumerate() {
    if line_i == 0 {
      continue; // skip drawing
    }
    if l.len() == 0 {
      continue; // skip empty lines
    }
    for cap in re.captures_iter(l) {
      last_board[board_i] = cap[1].parse::<u32>().unwrap();
      board_i += 1;
    }
    if board_i == 25 {
      boards.push(last_board.clone());
      let marked_squares: [bool; 25] = [
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false,
      ];
      squares_marked.push(marked_squares);
      board_i = 0;
    }
  }

  let mut winning_boards: Vec<usize> = Vec::new();
  let mut winning_numbers: Vec<u32> = Vec::new();
  for num in &drawing {
    for (i, board) in boards.iter().enumerate() {
      if winning_boards.contains(&i) {
        continue; // don’t keep marking winning boards
      }
      if board.contains(num) {
        let pos = board.iter().position(|s| s == num).unwrap();
        squares_marked[i][pos] = true;
        for s in 0..5 {
          let row_win = squares_marked[i][s * 5] == true
            && squares_marked[i][s * 5 + 1] == true
            && squares_marked[i][s * 5 + 2] == true
            && squares_marked[i][s * 5 + 3] == true
            && squares_marked[i][s * 5 + 4] == true;
          let col_win = squares_marked[i][s] == true
            && squares_marked[i][s + 5] == true
            && squares_marked[i][s + 10] == true
            && squares_marked[i][s + 15] == true
            && squares_marked[i][s + 20] == true;
          if row_win || col_win {
            winning_boards.push(i);
            winning_numbers.push(*num);
            break;
          }
        }
      }
    }
  }

  let mut first_winning_sum: u32 = 0;
  for (i, num) in boards[winning_boards[0]].iter().enumerate() {
    if !&squares_marked[winning_boards[0]][i] {
      first_winning_sum += num;
    }
  }

  println!("Part 1: {}", first_winning_sum * winning_numbers[0]);

  let last_i = winning_boards.len() - 1;
  let mut last_winning_sum: u32 = 0;
  for (i, num) in boards[winning_boards[last_i]].iter().enumerate() {
    if !&squares_marked[winning_boards[last_i]][i] {
      last_winning_sum += num;
    }
  }

  println!("Part 2: {}", last_winning_sum * winning_numbers[last_i]);
}
