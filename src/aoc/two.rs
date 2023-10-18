use crate::utils::read_file;
use std::io::BufRead;

const FILEPATH: &str = "src/aoc/inputs/two.txt";

// --- Day 2: Rock Paper Scissors ---

// ----------------------------------
// Part one
// ----------------------------------
pub fn part_one() {
  let reader = read_file(FILEPATH);
  let mut total_score: i32 = 0;

  for line in reader.lines() {
    let line = line.unwrap();
    let score = play_round_one(line);
    total_score += score;
  }

  println!("Day 2 | Part 1 -> Result: {}", total_score);
}

fn play_round_one(input: String) -> i32 {
  let arr = input.chars().collect::<Vec<char>>();
  let turns = arr.iter().filter(|&c| *c != ' ').collect::<Vec<&char>>();
  let mut score: i32 = 0;
  let my_move = turns[1];
  score += get_score_for_move_round_one(my_move);
  score += get_score_for_match_round_one(turns);

  score
}

fn get_score_for_move_round_one(my_move: &char) -> i32 {
  match my_move {
    'X' => 1,
    'Y' => 2,
    'Z' => 3,
    _ => 0,
  }
}

fn get_score_for_match_round_one(input: Vec<&char>) -> i32 {
  let her_move = *input[0];
  let my_move = *input[1];
  let score: i32;

  // Rock X A
  // Paper Y B
  // Scissors Z C

  match my_move {
    'X' => match her_move {
      'A' => score = 3,
      'B' => score = 0,
      'C' => score = 6,
      _ => score = 0,
    },
    'Y' => match her_move {
      'A' => score = 6,
      'B' => score = 3,
      'C' => score = 0,
      _ => score = 0,
    },
    'Z' => match her_move {
      'A' => score = 0,
      'B' => score = 6,
      'C' => score = 3,
      _ => score = 0,
    },
    _ => score = 0,
  }

  score
}

// ----------------------------------
// Part two
// ----------------------------------
pub fn part_two() {
  let reader = read_file(FILEPATH);
  let mut total_score: i32 = 0;

  for line in reader.lines() {
    let line = line.unwrap();
    let score = play_round_two(line);
    total_score += score;
  }

  println!("Day 2 | Part 2 -> Result: {}", total_score);
}

fn play_round_two(input: String) -> i32 {
  let arr = input.chars().collect::<Vec<char>>();
  let turns = arr.iter().filter(|&c| *c != ' ').collect::<Vec<&char>>();
  let mut score: i32 = 0;
  let my_move = turns[1];
  score += get_score_for_move_round_two(turns);
  score += get_score_for_match_round_two(my_move);

  score
}

fn get_score_for_move_round_two(input: Vec<&char>) -> i32 {
  let her_move = *input[0];
  let my_move = *input[1];
  let score: i32;

  // Rock A 1
  // Paper B 2
  // Scissors C 3

  // Lose X
  // Draw Y
  // Win Z

  match my_move {
    'X' => match her_move {
      'A' => score = 3,
      'B' => score = 1,
      'C' => score = 2,
      _ => score = 0,
    },
    'Y' => match her_move {
      'A' => score = 1,
      'B' => score = 2,
      'C' => score = 3,
      _ => score = 0,
    },
    'Z' => match her_move {
      'A' => score = 2,
      'B' => score = 3,
      'C' => score = 1,
      _ => score = 0,
    },
    _ => score = 0,
  }

  score
}

fn get_score_for_match_round_two(my_move: &char) -> i32 {
  match my_move {
    'X' => 0,
    'Y' => 3,
    'Z' => 6,
    _ => 0,
  }
}
