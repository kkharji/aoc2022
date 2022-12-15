mod input;

use std::{collections::HashSet, iter::FromIterator};

use itertools::Itertools;

use crate::{GetInput, Solve};

struct Case;

impl Solve for Case {
    fn part1(data: Option<Self::Input>) -> crate::Output {
        let chars = data.unwrap_or_else(Self::data).chars().collect_vec();
        let windows = chars.windows(4).enumerate();
        let result = windows.find_or_first(|(_, array)| {
            return HashSet::<&char>::from_iter(array.iter()).len() == 4;
        });
        result.map(|(i, _)| i + 4).unwrap_or_default().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_example_part1() {
        let value = Case::part1(String::from("bvwbjplbgvbhsrlpgdmjqwftvncz").into());
        assert_eq!(value, 5);
        let value = Case::part1(String::from("nppdvjthqldpwncqszvftbrmjlhg").into());
        assert_eq!(value, 6);
        let value = Case::part1(String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").into());
        assert_eq!(value, 10);
        let value = Case::part1(String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").into());
        assert_eq!(value, 11);
    }

    #[test]
    fn check_part1() {
        assert_eq!(Case::part1(None), 1542)
    }

    #[test]
    #[ignore = "no-implemented"]
    fn check_example_part2() {
        assert_eq!(Case::part2(Case::example().into()), 24000);
    }

    #[test]
    #[ignore = "no-implemented"]
    fn check_part2() {
        assert_eq!(Case::part2(None), 205381)
    }
}
