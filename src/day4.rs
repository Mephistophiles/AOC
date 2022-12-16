use std::str::FromStr;

use super::Solution;

struct Problem;

struct Range {
    from: i32,
    to: i32,
}

impl FromStr for Range {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (from, to) = s.split_once('-').unwrap();

        Ok(Range {
            from: from.parse().unwrap(),
            to: to.parse().unwrap(),
        })
    }
}
impl Range {
    fn from_line(line: String) -> (Range, Range) {
        let (part1, part2) = line.split_once(',').unwrap();

        (part1.parse().unwrap(), part2.parse().unwrap())
    }
    fn fully_contain(&self, other: &Self) -> bool {
        self.from >= other.from && self.to <= other.to
    }
    fn overlaps(&self, other: &Self) -> bool {
        self.from <= other.to && other.from <= self.to
    }
}

impl Solution for Problem {
    type Input = Vec<String>;
    type Result = i32;
    fn part1(&self, input: Vec<String>) -> i32 {
        input
            .into_iter()
            .map(Range::from_line)
            .filter(|(part1, part2)| part1.fully_contain(part2) || part2.fully_contain(part1))
            .count()
            .try_into()
            .unwrap()
    }

    fn part2(&self, input: Vec<String>) -> i32 {
        input
            .into_iter()
            .map(Range::from_line)
            .filter(|(part1, part2)| part1.overlaps(part2))
            .count()
            .try_into()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pre_part1_test() {
        let solution = Problem;
        let demo_input = crate::read_lines("day4_demo.txt");

        assert_eq!(solution.part1(demo_input), 2);
    }

    #[test]
    fn part1_test() {
        let solution = Problem;
        let demo_input = crate::read_lines("day4_part1.txt");

        assert_eq!(solution.part1(demo_input), 542);
    }

    #[test]
    fn pre_part2_test() {
        let solution = Problem;
        let demo_input = crate::read_lines("day4_demo.txt");

        assert_eq!(solution.part2(demo_input), 4);
    }

    #[test]
    fn part2_test() {
        let solution = Problem;
        let demo_input = crate::read_lines("day4_part1.txt");

        assert_eq!(solution.part2(demo_input), 900);
    }
}
