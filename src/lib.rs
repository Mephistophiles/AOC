mod day1;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

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
    fn part1(&self, input: Vec<String>) -> i32;
    fn part2(&self, input: Vec<String>) -> i32;
}
