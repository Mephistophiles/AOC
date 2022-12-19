use super::Solution;

struct Problem;

struct Day1Solve<const TOP: usize>;

impl<const TOP: usize> Day1Solve<TOP> {
    fn get_topn_results(&self, input: Vec<String>) -> i32 {
        let mut top: [i32; TOP] = [0; TOP];

        for groups in input.split(|pred| pred.is_empty()) {
            let group: i32 = groups.iter().map(|s| s.parse::<i32>().unwrap()).sum();
            let mut rem = group;

            for idx in (0..top.len()).rev() {
                if top[idx] < group {
                    std::mem::swap(&mut top[idx], &mut rem);
                }
            }
        }

        top.iter().sum()
    }
}

impl Solution for Problem {
    type Input = Vec<String>;
    type Result = i32;
    fn part1(&mut self, input: Vec<String>) -> i32 {
        Day1Solve::<1>.get_topn_results(input)
    }

    fn part2(&mut self, input: Vec<String>) -> i32 {
        Day1Solve::<3>.get_topn_results(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pre_part1_test() {
        let mut solution = Problem;
        let demo_input = crate::read_lines("day1_demo.txt");

        assert_eq!(solution.part1(demo_input), 24000);
    }

    #[test]
    fn part1_test() {
        let mut solution = Problem;
        let demo_input = crate::read_lines("day1_part1.txt");

        assert_eq!(solution.part1(demo_input), 67658);
    }

    #[test]
    fn pre_part2_test() {
        let mut solution = Problem;
        let demo_input = crate::read_lines("day1_demo.txt");

        assert_eq!(solution.part2(demo_input), 45000);
    }

    #[test]
    fn part2_test() {
        let mut solution = Problem;
        let demo_input = crate::read_lines("day1_part1.txt");

        assert_eq!(solution.part2(demo_input), 200158);
    }
}
