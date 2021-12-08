use std::fs;
use std::path::Path;

pub fn read_file(filepath: String) -> String {
  let msg = format!("couldn’t locate {}", filepath);
  return fs::read_to_string(Path::new(&filepath)).expect(&msg);
}
