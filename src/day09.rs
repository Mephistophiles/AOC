use std::collections::HashSet;
use std::ops::Sub;

use itertools::Itertools;

use super::Solution;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn parse(s: String) -> (Self, usize) {
        let (direction, count) = s.split_once(' ').unwrap();
        let count = count.parse().unwrap();

        (
            match direction {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => unreachable!(),
            },
            count,
        )
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Sub<Coord> for Coord {
    type Output = Coord;

    fn sub(self, rhs: Coord) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

struct Problem;

impl Problem {
    fn parse(input: Vec<String>) -> Vec<(Direction, usize)> {
        input.into_iter().map(Direction::parse).collect()
    }

    fn solve(moves: Vec<(Direction, usize)>, heads: usize) -> usize {
        let mut knots = vec![Coord::new(0, 0); heads + 1];
        let mut visited = HashSet::new();

        visited.insert(Coord::new(0, 0));

        for (mov, amount) in moves {
            for _ in 0..amount {
                match mov {
                    Direction::Up => knots[0].y -= 1,
                    Direction::Down => knots[0].y += 1,
                    Direction::Left => knots[0].x -= 1,
                    Direction::Right => knots[0].x += 1,
                }

                for (head_idx, tail_idx) in (0..knots.len()).tuple_windows() {
                    let diff = knots[head_idx] - knots[tail_idx];

                    if diff.x.abs() > 1 || diff.y.abs() > 1 {
                        knots[tail_idx].x += diff.x.signum();
                        knots[tail_idx].y += diff.y.signum();

                        if tail_idx == knots.len() - 1 {
                            visited.insert(*knots.last().unwrap());
                        }
                    }
                }
            }
        }

        visited.len()
    }
}

impl Solution for Problem {
    type Input = Vec<String>;
    type Result = usize;
    fn part1(&mut self, input: Self::Input) -> Self::Result {
        Problem::solve(Problem::parse(input), 1)
    }
    fn part2(&mut self, input: Self::Input) -> Self::Result {
        Problem::solve(Problem::parse(input), 9)
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
