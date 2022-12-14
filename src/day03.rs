use std::collections::HashSet;

use super::Solution;

struct Problem;

impl Problem {
    const fn renormalize_char(ch: char) -> i32 {
        let code = ch as i32;
        match ch {
            'a'..='z' => code - 'a' as i32 + 1,
            'A'..='Z' => code - 'A' as i32 + 27,
            _ => unreachable!(),
        }
    }
}

impl Solution for Problem {
    type Input = Vec<String>;
    type Result = i32;
    fn part1(&mut self, input: Vec<String>) -> i32 {
        let mut sum = 0;

        for line in input {
            debug_assert_eq!(line.len() % 2, 0);
            let (part1, part2) = line.split_at(line.len() / 2);
            let part1 = part1.chars().collect::<HashSet<char>>();
            let part2 = part2.chars().collect::<HashSet<char>>();
            let intersection = part1.intersection(&part2);

            sum += intersection
                .map(|&ch| Self::renormalize_char(ch))
                .sum::<i32>()
        }

        sum
    }

    fn part2(&mut self, input: Vec<String>) -> i32 {
        let mut sum = 0;

        for line in input.chunks(3) {
            let (part1, part2, part3) = match line {
                [line1, line2, line3] => (line1, line2, line3),
                _ => unreachable!(),
            };
            let part1 = part1.chars().collect::<HashSet<char>>();
            let part2 = part2.chars().collect::<HashSet<char>>();
            let part3 = part3.chars().collect::<HashSet<char>>();
            let intersection: HashSet<_> = part1.intersection(&part2).copied().collect();
            let intersection = intersection.intersection(&part3);

            sum += intersection
                .map(|&ch| Self::renormalize_char(ch))
                .sum::<i32>()
        }

        sum
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
