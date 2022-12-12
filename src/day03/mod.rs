mod input;

use std::ops::Div;

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
}

impl Solve for Case {
    fn part1(data: Option<Self::Input>) -> crate::Output {
        data.unwrap_or_else(Self::data)
            .into_iter()
            .map(|chars| {
                let (a, b) = chars.split_at(chars.len().div(2));
                (a.to_vec(), b.to_vec())
            })
            .flat_map(|(a, b)| b.into_iter().find_or_first(|bchar| a.contains(&bchar)))
            .map(Self::get_char_score)
            .sum::<u32>()
            .into()
    }

    fn part2(data: Option<Self::Input>) -> crate::Output {
        data.unwrap_or_else(Self::data)
            .chunks(3)
            .flat_map(|group| {
                let [a, b, c] = &group else { return  None; };
                a.into_iter()
                    .find_or_first(|char| b.contains(char) && c.contains(char))
                    .map(ToOwned::to_owned)
            })
            .map(Self::get_char_score)
            .sum::<u32>()
            .into()
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
    fn check_example_part2() {
        assert_eq!(Case::part2(Case::example().into()), 70);
    }

    #[test]
    fn check_part1() {
        assert_eq!(Case::part1(None), 7727)
    }

    #[test]
    fn check_part2() {
        assert_eq!(Case::part2(None), 2609)
    }
}
