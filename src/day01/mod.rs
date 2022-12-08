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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_example() {
        let part1 = Case::part1(Case::example().into());
        assert_eq!(part1, 24000);
    }

    #[test]
    fn check_part1() {
        let result = Case::part1(None);
        assert_eq!(result, 70296)
    }

    #[test]
    fn check_part2() {
        let _result = Case::part2(None);
    }
}
