use super::Solution;

pub struct Day1;

impl Solution for Day1 {
    fn part1(&self, input: Vec<String>) -> i32 {
        let mut max_group = 0;

        for groups in input.split(|pred| pred.is_empty()) {
            let group = groups.iter().map(|s| s.parse::<i32>().unwrap()).sum();
            max_group = max_group.max(group);
        }

        max_group
    }

    fn part2(&self, input: Vec<String>) -> i32 {
        let mut top3 = [0, 0, 0];

        for groups in input.split(|pred| pred.is_empty()) {
            let group: i32 = groups.iter().map(|s| s.parse::<i32>().unwrap()).sum();
            let mut rem = group;

            for idx in (0..top3.len()).rev() {
                if top3[idx] < group {
                    std::mem::swap(&mut top3[idx], &mut rem);
                }
            }
        }

        top3.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pre_part1_test() {
        let solution = Day1;
        let demo_input = crate::read_lines("day1_demo.txt");

        assert_eq!(solution.part1(demo_input), 24000);
    }

    #[test]
    fn part1_test() {
        let solution = Day1;
        let demo_input = crate::read_lines("day1_part1.txt");

        assert_eq!(solution.part1(demo_input), 67658);
    }

    #[test]
    fn pre_part2_test() {
        let solution = Day1;
        let demo_input = crate::read_lines("day1_demo.txt");

        assert_eq!(solution.part2(demo_input), 45000);
    }

    #[test]
    fn part2_test() {
        let solution = Day1;
        let demo_input = crate::read_lines("day1_part1.txt");

        assert_eq!(solution.part2(demo_input), 200158);
    }
}
