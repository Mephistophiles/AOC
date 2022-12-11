use super::Solution;

pub struct Day2;

#[derive(Clone, Copy, PartialEq)]
enum RPS {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Clone, Copy, PartialEq)]
enum Result {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

const PLAY_MATRIX: [(RPS, RPS, Result); 9] = [
    (RPS::Rock, RPS::Rock, Result::Draw),
    (RPS::Rock, RPS::Paper, Result::Lose),
    (RPS::Rock, RPS::Scissors, Result::Win),
    (RPS::Paper, RPS::Rock, Result::Win),
    (RPS::Paper, RPS::Paper, Result::Draw),
    (RPS::Paper, RPS::Scissors, Result::Lose),
    (RPS::Scissors, RPS::Rock, Result::Lose),
    (RPS::Scissors, RPS::Paper, Result::Win),
    (RPS::Scissors, RPS::Scissors, Result::Draw),
];

fn parse(line: &str) -> (RPS, RPS) {
    let (opponent, my_turn) = line.split_once(' ').unwrap();

    let opponent = match opponent {
        "A" => RPS::Rock,
        "B" => RPS::Paper,
        "C" => RPS::Scissors,
        _ => unreachable!(),
    };
    let my_turn = match my_turn {
        "X" => RPS::Rock,
        "Y" => RPS::Paper,
        "Z" => RPS::Scissors,
        _ => unreachable!(),
    };

    (opponent, my_turn)
}

impl Solution for Day2 {
    fn part1(&self, input: Vec<String>) -> i32 {
        let mut score = 0;

        for part in input {
            let (opponent, my_turn) = parse(&part);

            let play_result = PLAY_MATRIX
                .iter()
                .find(|(a, b, _)| a == &my_turn && b == &opponent)
                .unwrap()
                .2;

            score += play_result as i32 + my_turn as i32;
        }

        score
    }

    fn part2(&self, input: Vec<String>) -> i32 {
        let mut score = 0;

        for part in input {
            let (opponent, play_result) = parse(&part);

            let play_result = match play_result {
                RPS::Rock => Result::Lose,
                RPS::Paper => Result::Draw,
                RPS::Scissors => Result::Win,
            };

            let my_turn = PLAY_MATRIX
                .iter()
                .find(|(_, b, c)| b == &opponent && c == &play_result)
                .unwrap()
                .0;

            score += my_turn as i32 + play_result as i32;
        }

        score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pre_part1_test() {
        let solution = Day2;
        let demo_input = crate::read_lines("day2_demo.txt");

        assert_eq!(solution.part1(demo_input), 15);
    }

    #[test]
    fn part1_test() {
        let solution = Day2;
        let demo_input = crate::read_lines("day2_part1.txt");

        assert_eq!(solution.part1(demo_input), 14375);
    }

    #[test]
    fn pre_part2_test() {
        let solution = Day2;
        let demo_input = crate::read_lines("day2_demo.txt");

        assert_eq!(solution.part2(demo_input), 12);
    }

    #[test]
    fn part2_test() {
        let solution = Day2;
        let demo_input = crate::read_lines("day2_part1.txt");

        assert_eq!(solution.part2(demo_input), 10274);
    }
}
