mod input;

use crate::{GetInput, Solve};
use itertools::Itertools;

struct Case;
impl Case {
    fn get_char_score(char: char) -> u32 {
        let is_upper = char.is_ascii_uppercase();
        let value_start = if is_upper { 27 } else { 1 };
        let ascii_start = if is_upper { 65 } else { 97 };
        let ascii_code = char as u32;
        ascii_code - ascii_start + value_start
    }

    fn find_common<T: std::cmp::PartialEq>((a, b): (Vec<T>, Vec<T>)) -> Option<T> {
        b.into_iter().find_or_first(|bchar| a.contains(&bchar))
    }
}

impl Solve for Case {
    fn part1(data: Option<Self::Input>) -> crate::Output {
        data.unwrap_or_else(Self::data)
            .into_iter()
            .flat_map(Self::find_common)
            .map(Self::get_char_score)
            .sum::<u32>()
            .into()
    }

    fn part2(_data: Option<Self::Input>) -> crate::Output {
        "`Part 2 is not yet implemented`".into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_example_part1() {
        assert_eq!(Case::part1(Case::example().into()), 157);
    }

    #[test]
    #[ignore = "no-implemented"]
    fn check_example_part2() {
        assert_eq!(Case::part2(Case::example().into()), 24000);
    }

    #[test]
    fn check_part1() {
        assert_eq!(Case::part1(None), 7727)
    }

    #[test]
    #[ignore = "no-implemented"]
    fn check_part2() {
        assert_eq!(Case::part2(None), 205381)
    }
}
