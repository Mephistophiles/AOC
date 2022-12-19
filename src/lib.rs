mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn read_file<P: AsRef<Path>>(path: P) -> String {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("data")
        .join(path.as_ref());
    std::fs::read_to_string(file).unwrap()
}

pub fn read_lines<P: AsRef<Path>>(path: P) -> Vec<String> {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("data")
        .join(path.as_ref());
    let file = File::open(file).expect("File should exists");
    let mut buf_read = BufReader::new(file);
    let mut out = vec![];

    loop {
        let mut line = String::new();
        let nbytes = buf_read.read_line(&mut line).unwrap();

        if nbytes == 0 {
            break out;
        }

        if line.ends_with('\n') {
            line.truncate(line.len() - 1);
        }
        out.push(line);
    }
}

pub trait Solution {
    type Input;
    type Result;
    fn part1(&mut self, input: Self::Input) -> Self::Result;
    fn part2(&mut self, input: Self::Input) -> Self::Result;
}
