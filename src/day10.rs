use std::str::FromStr;

use crate::Void;

use super::Solution;

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i32),
}

impl FromStr for Instruction {
    type Err = Void;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (instruction, arg) = s.split_once(' ').unwrap_or((s, ""));

        Ok(match instruction {
            "addx" => Instruction::Addx(arg.parse().expect("Number")),
            "noop" => Instruction::Noop,
            _ => unreachable!(),
        })
    }
}

struct Problem;

impl Problem {
    fn parse(instr_set: Vec<String>) -> Vec<Instruction> {
        instr_set
            .into_iter()
            .filter_map(|line| line.parse().ok())
            .collect()
    }

    fn solve(input: Vec<String>) -> Vec<i32> {
        let mut x = vec![1, 1];

        for instruction in Problem::parse(input) {
            x.push(*x.last().unwrap());

            match instruction {
                Instruction::Noop => {}
                Instruction::Addx(v) => x.push(x.last().unwrap() + v),
            }
        }

        x
    }
}

impl Solution for Problem {
    type Input = Vec<String>;
    type Result = String;
    fn part1(&mut self, input: Self::Input) -> Self::Result {
        let x = Problem::solve(input);

        (20..=220)
            .step_by(40)
            .map(|i| i as i32 * x[i])
            .sum::<i32>()
            .to_string()
    }
    fn part2(&mut self, input: Self::Input) -> Self::Result {
        let x = Problem::solve(input);

        let mut s = String::new();
        for row in 0..6 {
            s += "\n";
            for (pos, cycle) in ((40 * row + 1)..=(40 * (row + 1))).enumerate() {
                let sprite = x[cycle];
                if sprite - 1 <= pos as i32 && pos as i32 <= sprite + 1 {
                    s.push('#');
                } else {
                    s.push('.');
                }
            }
        }

        s
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
