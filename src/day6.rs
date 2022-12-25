use std::marker::PhantomData;

use super::Solution;

struct Problem<'a> {
    _phantom: PhantomData<&'a str>,
}

impl<'a> Problem<'a> {
    fn char_to_code(c: u8) -> u8 {
        debug_assert!((b'a'..=b'z').contains(&c));

        c - b'a'
    }

    fn find_marker(input: &str, window_size: usize) -> usize {
        input
            .as_bytes()
            .windows(window_size)
            .position(|window| {
                let mut cache: u32 = 0;

                for &item in window {
                    let bit = 1 << Problem::char_to_code(item);
                    if cache & bit != 0 {
                        return false;
                    }

                    cache |= bit;
                }

                true
            })
            .unwrap()
            + window_size
    }
}

impl<'a> Solution for Problem<'a> {
    type Input = &'a str;
    type Result = usize;
    fn part1(&mut self, input: &str) -> usize {
        Problem::find_marker(input, 4)
    }

    fn part2(&mut self, input: &str) -> usize {
        Problem::find_marker(input, 14)
    }
}

#[cfg(test)]
mod tests {
    use crate::problem;

    use super::*;

    #[test]
    fn pre_part1_test() {
        let mut solution = Problem {
            _phantom: PhantomData,
        };

        insta::assert_debug_snapshot!(solution.part1("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        insta::assert_debug_snapshot!(solution.part1("nppdvjthqldpwncqszvftbrmjlhg"));
        insta::assert_debug_snapshot!(solution.part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        insta::assert_debug_snapshot!(solution.part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }

    #[test]
    fn part1_test() {
        let mut solution = Problem {
            _phantom: PhantomData,
        };
        let demo = problem!();

        insta::assert_debug_snapshot!(solution.part1(&demo));
    }

    #[test]
    fn pre_part2_test() {
        let mut solution = Problem {
            _phantom: PhantomData,
        };

        insta::assert_debug_snapshot!(solution.part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        insta::assert_debug_snapshot!(solution.part2("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        insta::assert_debug_snapshot!(solution.part2("nppdvjthqldpwncqszvftbrmjlhg"));
        insta::assert_debug_snapshot!(solution.part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        insta::assert_debug_snapshot!(solution.part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }

    #[test]
    fn part2_test() {
        let mut solution = Problem {
            _phantom: PhantomData,
        };

        let demo = problem!();

        insta::assert_debug_snapshot!(solution.part2(&demo));
    }
}
