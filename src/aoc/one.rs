use crate::utils::read_file;
use std::io::BufRead;

const FILEPATH: &str = "src/aoc/inputs/one.txt";

pub fn part_one() {
  let reader = read_file(FILEPATH);
  let mut highest_sum = 0;
  let mut sum = 0;

  for line in reader.lines() {
    let line = line.unwrap().parse().unwrap_or(0);

    if line == 0 {
      if sum > highest_sum {
        highest_sum = sum;
      }
      sum = 0;
    } else {
      sum += line;
    }
  }

  println!("Day 1 | Part 1 -> Result: {}", highest_sum);
}

pub fn part_two() {
  let reader = read_file(FILEPATH);

  // Get sum for each elf
  let mut sum: i32 = 0;
  let mut elf_sums: Vec<i32> = Vec::new();
  for line in reader.lines() {
    let line = line.unwrap().parse().unwrap_or(0);
    if line > 0 {
      sum += line;
    } else {
      elf_sums.push(sum);
      sum = 0;
    }
  }

  // Sort the elf sums in reverse order
  elf_sums.sort_by(|a, b| b.cmp(a));

  let mut result: i32 = 0;

  result += elf_sums[0];
  result += elf_sums[1];
  result += elf_sums[2];

  println!("Day 1 | Part 2 -> Result: {}", result);
}
