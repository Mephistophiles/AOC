mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

pub fn read_file<P: AsRef<Path>>(path: P) -> String {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("data")
        .join(path.as_ref());
    std::fs::read_to_string(file).unwrap()
}

fn read_lines(file: PathBuf) -> Vec<String> {
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

fn get_file(day: usize, t: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("data")
        .join(format!("day{day}_{t}.txt"))
}

pub fn get_demo(day: usize) -> String {
    let file = get_file(day, "demo");
    std::fs::read_to_string(file).unwrap()
}

pub fn get_problem(day: usize) -> String {
    let file = get_file(day, "problem");
    std::fs::read_to_string(file).unwrap()
}

pub fn get_demo_lines(day: usize) -> Vec<String> {
    let file = get_file(day, "demo");
    read_lines(file)
}

pub fn get_problem_lines(day: usize) -> Vec<String> {
    let file = get_file(day, "problem");
    read_lines(file)
}

pub trait Solution {
    type Input;
    type Result;
    fn part1(&mut self, input: Self::Input) -> Self::Result;
    fn part2(&mut self, input: Self::Input) -> Self::Result;
}
