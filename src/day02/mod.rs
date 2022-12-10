use crate::util::GetInput;
use crate::Solve;

mod input;

pub struct Day02;
type Case = Day02;

type Input = Vec<(char, char)>;

// A/Rock = 1, B/Paper = 2, C/Scissors = 3,, Win = 6, Draw = 3, lost
impl Case {
    fn get_val(key: char) -> u16 {
        match key {
            'A' => 1,
            'B' => 2,
            'C' => 3,
            _ => unreachable!("expected A,B or C"),
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_example() {
        let part1 = Case::part1(Case::example().into());
        assert_eq!(part1, 15);
    }

    #[test]
    fn check_part1() {
        let result = Case::part1(None);
        assert_eq!(result, 11767)
    }

    #[test]
    #[ignore = "not-implemented"]
    fn check_part2() {
        let result = Case::part2(None);
        assert_eq!(result, 00000)
    }
}
