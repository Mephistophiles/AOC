use super::Solution;

struct Problem;

#[derive(Debug)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

impl Problem {
    fn parse_stacks(stacks: &[String]) -> Vec<Vec<&str>> {
        let num_of_stacks = stacks.last().unwrap().split_whitespace().count();
        let mut out = vec![Vec::new(); num_of_stacks];

        for line in stacks.iter().take(stacks.len() - 1).rev() {
            for idx in 0..num_of_stacks {
                let stack = &line[idx * 4 + 1..idx * 4 + 2];

                if stack != " " {
                    out[idx].push(stack)
                }
            }
        }

        out
    }

    fn parse_moves(moves: &[String]) -> Vec<Move> {
        moves
            .iter()
            .map(|line| {
                let mut items = line.split_whitespace();
                items.next();
                let count = items.next().unwrap().parse().unwrap();
                items.next();
                let from = items.next().unwrap().parse().unwrap();
                items.next();
                let to = items.next().unwrap().parse().unwrap();

                Move { count, from, to }
            })
            .collect()
    }

    fn crane_in(stacks: &mut [Vec<&str>], moves: &[Move], all_crates_at_once: bool) {
        let mut buf = vec![];
        for mov in moves {
            let mut rem = mov.count;

            while rem != 0 {
                let count = if all_crates_at_once { rem } else { 1 };
                let from = &mut stacks[mov.from - 1];
                buf.extend(from.drain(from.len() - count..));
                stacks[mov.to - 1].append(&mut buf);
                rem -= count;
            }
        }
    }
    fn solve(input: Vec<String>, all_crates_at_once: bool) -> String {
        let mut splits = input.split(|line| line.is_empty());
        let mut stacks = Self::parse_stacks(splits.next().unwrap());
        let moves = Self::parse_moves(splits.next().unwrap());

        Self::crane_in(&mut stacks, moves.as_ref(), all_crates_at_once);

        let mut out = String::with_capacity(stacks.len());
        for mut stack in stacks.into_iter() {
            if let Some(item) = stack.pop() {
                out.push_str(item);
            }
        }

        out
    }
}

impl Solution for Problem {
    type Input = Vec<String>;
    type Result = String;
    fn part1(&mut self, input: Vec<String>) -> String {
        Self::solve(input, false)
    }

    fn part2(&mut self, input: Vec<String>) -> String {
        Self::solve(input, true)
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
