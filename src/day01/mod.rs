use itertools::Itertools;

use crate::util::GetInput;
use crate::Solve;

mod input;

struct Case;

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
    fn check_example_part1() {
        assert_eq!(Case::part1(Case::example().into()), 24000);
    }
    #[test]
    fn check_example_part2() {
        assert_eq!(Case::part2(Case::example().into()), 45000);
    }

    #[test]
    fn check_part1() {
        assert_eq!(Case::part1(None), 70296)
    }

    #[test]
    fn check_part2() {
        assert_eq!(Case::part2(None), 205381)
    }
}
