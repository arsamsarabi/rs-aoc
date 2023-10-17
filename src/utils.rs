use std::fs::File;
use std::io::BufReader;

pub fn read_file(filepath: &str) -> BufReader<File> {
  let file = File::open(filepath).expect("Input file not found");
  let reader = BufReader::new(file);

  return reader;
}
