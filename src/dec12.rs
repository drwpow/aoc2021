use crate::utils;
use std::collections::HashMap;
use std::collections::HashSet;

type Tree = HashMap<String, HashSet<String>>;

pub fn run() {
  let filepath = "data/input12.txt".to_owned();
  let mut cave_tree: Tree = HashMap::new();
  utils::read_file(filepath)
    .split("\n")
    .filter(|l| l.len() > 0)
    .for_each(|l| {
      let nodes: Vec<String> = l.split("-").map(|n| n.to_string()).collect();
      if nodes[0] != "end" && nodes[1] != "start" {
        let branch = cave_tree
          .entry(nodes[0].to_string())
          .or_insert(HashSet::new());
        branch.insert(nodes[1].to_string());
      }
      if nodes[1] != "end" && nodes[0] != "start" {
        let branch = cave_tree
          .entry(nodes[1].to_string())
          .or_insert(HashSet::new());
        branch.insert(nodes[0].to_string());
      }
    });

  let first_crawl = crawl(&cave_tree, None);

  println!("Part 1: {}", first_crawl.len());

  let second_crawl = crawl2(&cave_tree, None);

  println!("Part 2: {}", second_crawl.len());
}

fn crawl(t: &Tree, paths: Option<&Vec<Vec<String>>>) -> Vec<Vec<String>> {
  let mut has_unfinished: bool = false;
  let mut new_paths: Vec<Vec<String>> = vec![];

  let crawled: Vec<Vec<String>> = match paths {
    None => vec![vec!["start".to_owned()]],
    Some(p) => p.to_vec(),
  };
  for path in &crawled {
    let current = path.last().unwrap();
    if *current == "end" {
      new_paths.push(path.clone());
      continue; // if path has ended, don’t continue
    }
    let next_nodes = t.get(current).unwrap();
    for node in next_nodes {
      // don’t revisit “small” caves
      let is_lower = *node == *node.to_lowercase();
      if is_lower && path.contains(node) {
        continue;
      }
      has_unfinished = true;
      // create clone of Vector (is this dumb?)
      let mut new_path: Vec<String> = Vec::new();
      for step in path {
        new_path.push(step.to_string());
      }
      new_path.push(node.to_string());
      new_paths.push(new_path);
    }
  }

  // if there are some undended paths, continue crawling
  if has_unfinished {
    return crawl(t, Some(&new_paths));
  }

  return new_paths;
}

// copyin cuz lazy
fn crawl2(t: &Tree, paths: Option<&Vec<Vec<String>>>) -> Vec<Vec<String>> {
  let mut has_unfinished: bool = false;
  let mut new_paths: Vec<Vec<String>> = vec![];

  let crawled: Vec<Vec<String>> = match paths {
    None => vec![vec!["start".to_owned()]],
    Some(p) => p.to_vec(),
  };
  for path in &crawled {
    let current = path.last().unwrap();
    if *current == "end" {
      new_paths.push(path.clone());
      continue; // if path has ended, don’t continue
    }

    // if one small cave has been visited twice already, we’re “maxed”
    let mut is_maxed = false;
    let mut small_cave_counts: HashMap<String, i32> = HashMap::new();
    for s in path {
      if s == "start" || s == "end" || *s != s.to_lowercase() {
        continue;
      }
      let counter = small_cave_counts.entry(s.to_string()).or_insert(0);
      *counter += 1;
    }
    if small_cave_counts.values().filter(|c| *c >= &2).count() == 1 {
      is_maxed = true;
    }

    let next_nodes = t.get(current).unwrap();
    for node in next_nodes {
      // if this is a small cave and we’re maxed out, skip
      if is_maxed && small_cave_counts.get(node).or(Some(&0)).unwrap() > &0 {
        continue;
      }
      has_unfinished = true;
      // create clone of Vector (is this dumb?)
      let mut new_path: Vec<String> = Vec::new();
      for step in path {
        new_path.push(step.to_string());
      }
      new_path.push(node.to_string());
      new_paths.push(new_path);
    }
  }

  // if there are some undended paths, continue crawling
  if has_unfinished {
    return crawl2(t, Some(&new_paths));
  }

  return new_paths;
}
