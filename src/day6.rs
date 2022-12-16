use super::Solution;

struct Problem;

impl Problem {
    fn char_to_code(c: u8) -> u8 {
        debug_assert!((b'a'..=b'z').contains(&c));

        c - b'a'
    }

    fn find_marker(input: String, window_size: usize) -> usize {
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

impl Solution for Problem {
    type Input = String;
    type Result = usize;
    fn part1(&self, input: String) -> usize {
        Problem::find_marker(input, 4)
    }

    fn part2(&self, input: String) -> usize {
        Problem::find_marker(input, 14)
    }
}

#[cfg(test)]
mod tests {
    use crate::read_file;

    use super::*;

    #[test]
    fn pre_part1_test() {
        let solution = Problem;

        assert_eq!(
            solution.part1("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()),
            5
        );
        assert_eq!(
            solution.part1("nppdvjthqldpwncqszvftbrmjlhg".to_string()),
            6
        );
        assert_eq!(
            solution.part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string()),
            10
        );
        assert_eq!(
            solution.part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string()),
            11
        );
    }

    #[test]
    fn part1_test() {
        let solution = Problem;
        let demo = read_file("day6_part1.txt");

        assert_eq!(solution.part1(demo), 1155);
    }

    #[test]
    fn pre_part2_test() {
        let solution = Problem;

        assert_eq!(
            solution.part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string()),
            19
        );
        assert_eq!(
            solution.part2("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()),
            23
        );
        assert_eq!(
            solution.part2("nppdvjthqldpwncqszvftbrmjlhg".to_string()),
            23
        );
        assert_eq!(
            solution.part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string()),
            29
        );
        assert_eq!(
            solution.part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string()),
            26
        );
    }

    #[test]
    fn part2_test() {
        let solution = Problem;

        let demo = read_file("day6_part1.txt");

        assert_eq!(solution.part2(demo), 2789);
    }
}
