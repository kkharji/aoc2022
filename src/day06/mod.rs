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
    fn part2(data: Option<Self::Input>) -> crate::Output {
        let chars = data.unwrap_or_else(Self::data).chars().collect_vec();
        let windows = chars.windows(14).enumerate();
        let result = windows.find_or_first(|(_, array)| {
            return HashSet::<&char>::from_iter(array.iter()).len() == 14;
        });
        result.map(|(i, _)| i + 14).unwrap_or_default().into()
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
    fn check_example_part2() {
        [
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
        ]
        .into_iter()
        .map(|(s, i)| (s.to_string(), i))
        .for_each(|(value, expected)| {
            let result = Case::part2(value.clone().into());
            assert_eq!(
                result, expected,
                "expected '{value}' to be '{expected}' but got '{result}'"
            );
        });
    }

    #[test]
    fn check_part2() {
        assert_eq!(Case::part2(None), 3153)
    }
}
