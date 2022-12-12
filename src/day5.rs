use std::collections::VecDeque;

use super::Solution;

struct Problem;

#[derive(Debug)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

impl Problem {
    fn parse_stacks(stacks: &[String]) -> Vec<VecDeque<&str>> {
        let num_of_stacks = stacks.last().unwrap().split_whitespace().count();
        let mut out = vec![VecDeque::new(); num_of_stacks];

        for line in stacks.iter().take(stacks.len() - 1) {
            for idx in 0..num_of_stacks {
                let stack = &line[idx * 4 + 1..idx * 4 + 2];

                if stack != " " {
                    out[idx].push_back(stack)
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

    fn process_part1(stacks: &mut [VecDeque<&str>], moves: &[Move]) {
        for mov in moves {
            for _ in 0..mov.count {
                let item = stacks[mov.from - 1].pop_front().unwrap();
                stacks[mov.to - 1].push_front(item);
            }
        }
    }

    fn process_part2(stacks: &mut [VecDeque<&str>], moves: &[Move]) {
        for mov in moves {
            let items: VecDeque<&str> = stacks[mov.from - 1].drain(0..mov.count).collect();
            for item in items.into_iter().rev() {
                stacks[mov.to - 1].push_front(item);
            }
        }
    }
}

impl Solution for Problem {
    type Result = String;
    fn part1(&self, input: Vec<String>) -> String {
        let mut splits = input.split(|line| line.is_empty());
        let mut stacks = Self::parse_stacks(splits.next().unwrap());
        let moves = Self::parse_moves(splits.next().unwrap());

        Self::process_part1(&mut stacks, moves.as_ref());

        let mut out = String::with_capacity(stacks.len());
        for mut stack in stacks.into_iter() {
            if let Some(item) = stack.pop_front() {
                out.push_str(item);
            }
        }

        out
    }

    fn part2(&self, input: Vec<String>) -> String {
        let mut splits = input.split(|line| line.is_empty());
        let mut stacks = Self::parse_stacks(splits.next().unwrap());
        let moves = Self::parse_moves(splits.next().unwrap());

        Self::process_part2(&mut stacks, moves.as_ref());

        let mut out = String::with_capacity(stacks.len());
        for mut stack in stacks.into_iter() {
            if let Some(item) = stack.pop_front() {
                out.push_str(item);
            }
        }

        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pre_part1_test() {
        let solution = Problem;
        let demo_input = crate::read_lines("day5_demo.txt");

        assert_eq!(solution.part1(demo_input), "CMZ".to_string());
    }

    #[test]
    fn part1_test() {
        let solution = Problem;
        let demo_input = crate::read_lines("day5_part1.txt");

        assert_eq!(solution.part1(demo_input), "CFFHVVHNC".to_string());
    }

    #[test]
    fn pre_part2_test() {
        let solution = Problem;
        let demo_input = crate::read_lines("day5_demo.txt");

        assert_eq!(solution.part2(demo_input), "MCD".to_string());
    }

    #[test]
    fn part2_test() {
        let solution = Problem;
        let demo_input = crate::read_lines("day5_part1.txt");

        assert_eq!(solution.part2(demo_input), "FSZWBPTBG".to_string());
    }
}
