use super::Solution;

#[cfg(test)]
const DAY: usize = 2;

struct Problem;

#[derive(Clone, Copy, PartialEq)]
enum Rps {
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

const PLAY_MATRIX: [(Rps, Rps, Result); 9] = [
    (Rps::Rock, Rps::Rock, Result::Draw),
    (Rps::Rock, Rps::Paper, Result::Lose),
    (Rps::Rock, Rps::Scissors, Result::Win),
    (Rps::Paper, Rps::Rock, Result::Win),
    (Rps::Paper, Rps::Paper, Result::Draw),
    (Rps::Paper, Rps::Scissors, Result::Lose),
    (Rps::Scissors, Rps::Rock, Result::Lose),
    (Rps::Scissors, Rps::Paper, Result::Win),
    (Rps::Scissors, Rps::Scissors, Result::Draw),
];

fn parse(line: &str) -> (Rps, Rps) {
    let (opponent, my_turn) = line.split_once(' ').unwrap();

    let opponent = match opponent {
        "A" => Rps::Rock,
        "B" => Rps::Paper,
        "C" => Rps::Scissors,
        _ => unreachable!(),
    };
    let my_turn = match my_turn {
        "X" => Rps::Rock,
        "Y" => Rps::Paper,
        "Z" => Rps::Scissors,
        _ => unreachable!(),
    };

    (opponent, my_turn)
}

impl Solution for Problem {
    type Input = Vec<String>;
    type Result = i32;
    fn part1(&mut self, input: Vec<String>) -> i32 {
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

    fn part2(&mut self, input: Vec<String>) -> i32 {
        let mut score = 0;

        for part in input {
            let (opponent, play_result) = parse(&part);

            let play_result = match play_result {
                Rps::Rock => Result::Lose,
                Rps::Paper => Result::Draw,
                Rps::Scissors => Result::Win,
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
        let mut solution = Problem;
        let demo_input = crate::get_demo_lines(DAY);

        assert_eq!(solution.part1(demo_input), 15);
    }

    #[test]
    fn part1_test() {
        let mut solution = Problem;
        let demo_input = crate::get_problem_lines(DAY);

        assert_eq!(solution.part1(demo_input), 14375);
    }

    #[test]
    fn pre_part2_test() {
        let mut solution = Problem;
        let demo_input = crate::get_demo_lines(DAY);

        assert_eq!(solution.part2(demo_input), 12);
    }

    #[test]
    fn part2_test() {
        let mut solution = Problem;
        let demo_input = crate::get_problem_lines(DAY);

        assert_eq!(solution.part2(demo_input), 10274);
    }
}
