use crate::util::GetInput;
use crate::Solve;

mod input;

pub struct Day01;

type Input = Vec<Vec<i32>>;

impl Solve for Day01 {
    fn part1() -> crate::Output {
        Day01::data()
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
        let result = Day01::example().iter().map(|cals| cals.iter().sum()).max();
        assert_eq!(result, Some(24000));
    }

    #[test]
    fn check_part1() {
        let result = Day01::part1();
        assert_eq!(result, 70296)
    }

    #[test]
    fn check_part2() {
        let _result = Day01::part1();
    }
}
