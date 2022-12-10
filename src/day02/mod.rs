mod input;
use crate::{GetInput, Solve};

pub struct Case;

// A/Rock = 1, B/Paper = 2, C/Scissors = 3,, Win = 6, Draw = 3, lost
impl Case {
    fn get_val(key: char) -> u16 {
        match key {
            'A' => 1,
            'B' => 2,
            'C' => 3,
            _ => unreachable!("expected A, B or C"),
        }
    }

    fn xyz_to_abc(key: char) -> char {
        match key {
            'X' => 'A',
            'Y' => 'B',
            'Z' => 'C',
            l => unreachable!("expect x or y or z, got {l}"),
        }
    }

    fn is_win(a: char, b: char, lhs: bool) -> bool {
        if let ('A', 'B') | ('B', 'C') | ('C', 'A') = (a, b) {
            !lhs
        } else {
            lhs
        }
    }

    fn predict_next_move(a: char, b: char) -> char {
        match b {
            'X' => match a {
                'A' => 'C',
                'B' => 'A',
                'C' => 'B',
                _ => unreachable!("expected A, B or C"),
            },
            'Z' => match a {
                'A' => 'B',
                'B' => 'C',
                'C' => 'A',
                _ => unreachable!("expected A, B or C"),
            },
            'Y' => a,
            l => unreachable!("expect x or y or z, got {l}"),
        }
    }
}

impl Solve for Case {
    fn part1(data: Option<Self::Input>) -> crate::Output {
        data.unwrap_or_else(Self::data)
            .into_iter()
            .map(|(a, b)| {
                let b = Self::xyz_to_abc(b);
                let mut result = Self::get_val(b);
                if Self::is_win(a, b, false) {
                    result += 6
                } else if a == b {
                    result += 3;
                }
                result
            })
            .sum::<u16>()
            .into()
    }

    fn part2(data: Option<Self::Input>) -> crate::Output {
        data.unwrap_or_else(Self::data)
            .into_iter()
            .map(|(a, b)| {
                let bval = Self::get_val(Self::predict_next_move(a, b));
                match b {
                    'X' => bval + 0,
                    'Y' => bval + 3,
                    'Z' => bval + 6,
                    l => unreachable!("expect x or y or z, got {l}"),
                }
            })
            .sum::<u16>()
            .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_example_part1() {
        assert_eq!(Case::part1(Case::example().into()), 15);
    }

    #[test]
    fn check_example_part2() {
        assert_eq!(Case::part2(Case::example().into()), 12);
    }

    #[test]
    fn check_part1() {
        assert_eq!(Case::part1(None), 11767)
    }

    #[test]
    fn check_part2() {
        assert_eq!(Case::part2(None), 13886)
    }
}
