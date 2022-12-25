use super::Solution;

struct Problem;

impl Problem {
    fn parse(input: Vec<String>) -> Vec<Vec<usize>> {
        input
            .into_iter()
            .map(|line| {
                line.chars()
                    .filter_map(|c| c.to_digit(10))
                    .map(|c| c as usize)
                    .collect()
            })
            .collect()
    }

    fn process_slice((local_best, height): &mut (usize, usize), iter: impl Iterator<Item = usize>) {
        let mut score = 0;

        for i in iter {
            score += 1;
            if i >= *height {
                break;
            }
        }

        *local_best *= score;
    }
}

impl Solution for Problem {
    type Input = Vec<String>;
    type Result = usize;
    fn part1(&mut self, input: Vec<String>) -> usize {
        let forest = Problem::parse(input);
        debug_assert!(forest.iter().all(|t| t.len() == forest[0].len()));

        let (h, w) = (forest.len(), forest[0].len());
        (0..h)
            .flat_map(move |h| (0..w).map(move |w| (h, w)))
            .map(|(row, col)| (row, col, forest[row][col]))
            .filter(|&(row, col, height)| {
                forest[..row].iter().all(|x| x[col] < height)
                    || forest[row][..col].iter().all(|&x| x < height)
                    || forest[row + 1..].iter().all(|x| x[col] < height)
                    || forest[row][col + 1..].iter().all(|&x| x < height)
            })
            .count()
    }

    fn part2(&mut self, input: Vec<String>) -> usize {
        let mut count = 0;
        let forest = Problem::parse(input);
        debug_assert!(forest.iter().all(|t| t.len() == forest[0].len()));

        let (h, w) = (forest.len(), forest[0].len());

        for row in 0..h {
            for col in 0..w {
                let mut ctx = (1, forest[row][col]);
                Problem::process_slice(&mut ctx, forest[..row].iter().map(|x| x[col]).rev());
                Problem::process_slice(&mut ctx, forest[row][..col].iter().rev().copied());
                Problem::process_slice(&mut ctx, forest[row + 1..].iter().map(|x| x[col]));
                Problem::process_slice(&mut ctx, forest[row][col + 1..].iter().copied());
                count = count.max(ctx.0);
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use crate::{demo_lines, problem_lines};

    use super::*;

    #[test]
    fn pre_part1_test() {
        let mut solution = Problem;
        let demo_input = demo_lines!();

        assert_eq!(solution.part1(demo_input), 21);
    }

    #[test]
    fn part1_test() {
        let mut solution = Problem;
        let demo_input = problem_lines!();

        assert_eq!(solution.part1(demo_input), 1546);
    }

    #[test]
    fn pre_part2_test() {
        let mut solution = Problem;
        let demo_input = demo_lines!();

        assert_eq!(solution.part2(demo_input), 8);
    }

    #[test]
    fn part2_test() {
        let mut solution = Problem;
        let demo_input = problem_lines!();

        assert_eq!(solution.part2(demo_input), 519064);
    }
}
