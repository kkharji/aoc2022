use itertools::Itertools;

use crate::util::GetInput;
use crate::Solve;

mod input;

pub struct Day01;

type Case = Day01;
type Input = Vec<Vec<i32>>;

impl Solve for Case {
    fn part1(data: Option<Self::Input>) -> crate::Output {
        data.unwrap_or_else(Self::data)
            .into_iter()
            .map(|cals| cals.into_iter().sum::<i32>())
            .max()
            .unwrap_or_default()
            .into()
    }

    fn part2(data: Option<Self::Input>) -> crate::Output {
        data.unwrap_or_else(Self::data)
            .iter()
            .map(|cals| cals.iter().sum::<i32>())
            .sorted_by(|a, b| b.cmp(a))
            .take(3)
            .sum::<i32>()
            .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_example() {
        let part1 = Case::part1(Case::example().into());
        assert_eq!(part1, 24000);
        let part1 = Case::part2(Case::example().into());
        assert_eq!(part1, 45000);
    }

    #[test]
    fn check_part1() {
        let result = Case::part1(None);
        assert_eq!(result, 70296)
    }

    #[test]
    fn check_part2() {
        let result = Case::part2(None);
        assert_eq!(result, 205381)
    }
}
