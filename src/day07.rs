use std::collections::HashMap;
use std::path::{Path, PathBuf};

struct Problem;

use super::Solution;

fn process(input: Vec<String>) -> HashMap<PathBuf, usize> {
    let mut pwd = vec![];
    let mut dirs: HashMap<PathBuf, usize> = HashMap::new();

    for line in &input {
        if let Some(command) = line.strip_prefix("$ ") {
            let (command, data) = command.split_once(' ').unwrap_or((command, ""));
            match (command, data) {
                ("ls", "") => continue,
                ("cd", "..") => {
                    pwd.pop();
                }
                ("cd", x) => pwd.push(x),
                _ => unreachable!(),
            }
        } else {
            let (type_size, name) = line.split_once(' ').unwrap();

            match (type_size, name) {
                ("dir", _) => continue,
                (size, _) => {
                    let size = size.parse::<usize>().unwrap();

                    for idx in 0..pwd.len() {
                        let path = PathBuf::from_iter(&pwd[..=idx]);
                        *dirs.entry(path).or_insert(0) += size;
                    }
                }
            }
        }
    }
    dirs
}

impl Solution for Problem {
    type Input = Vec<String>;
    type Result = usize;
    fn part1(&mut self, input: Vec<String>) -> usize {
        let dirs = process(input);

        dirs.into_values().filter(|&size| size <= 100_000).sum()
    }

    fn part2(&mut self, input: Vec<String>) -> usize {
        const TOTAL_DISK: usize = 70_000_000;
        const NEEDED_UNUSED: usize = 30_000_000;

        let dirs = process(input);
        let used = dirs.get(Path::new("/")).unwrap();
        let unused = TOTAL_DISK - used;

        dirs.into_values()
            .filter(|&size| unused + size >= NEEDED_UNUSED)
            .min()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::{demo_lines, problem_lines};

    use super::*;

    #[test]
    fn part1_demo_test() {
        let mut solution = Problem;
        let demo_input = demo_lines!();

        insta::assert_debug_snapshot!(solution.part1(demo_input));
    }

    #[test]
    fn part1_problem_test() {
        let mut solution = Problem;
        let demo_input = problem_lines!();

        insta::assert_debug_snapshot!(solution.part1(demo_input));
    }

    #[test]
    fn part2_demo_test() {
        let mut solution = Problem;
        let demo_input = demo_lines!();

        insta::assert_debug_snapshot!(solution.part2(demo_input));
    }

    #[test]
    fn part2_problem_test() {
        let mut solution = Problem;
        let demo_input = problem_lines!();

        insta::assert_debug_snapshot!(solution.part2(demo_input));
    }
}
