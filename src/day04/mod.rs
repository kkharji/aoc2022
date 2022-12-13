mod input;

use crate::{GetInput, Solve};

struct Case;

impl input::SectionAssignmentPair {
    fn to_vecs(&self) -> (Vec<usize>, Vec<usize>) {
        let lhs: Vec<_> = (self.lhs.start..self.lhs.end + 1).collect();
        let rhs: Vec<_> = (self.rhs.start..self.rhs.end + 1).collect();
        if rhs.len() > lhs.len() {
            (rhs, lhs)
        } else {
            (lhs, rhs)
        }
    }
}

impl Solve for Case {
    fn part1(data: Option<Self::Input>) -> crate::Output {
        data.unwrap_or_else(Self::data)
            .into_iter()
            .filter(|sap| {
                let (lg, sh) = sap.to_vecs();
                sh.into_iter().all(|i| lg.contains(&i))
            })
            .count()
            .into()
    }

    fn part2(data: Option<Self::Input>) -> crate::Output {
        data.unwrap_or_else(Self::data)
            .into_iter()
            .filter(|sap| {
                let (lg, sh) = sap.to_vecs();
                sh.into_iter().any(|i| lg.contains(&i))
            })
            .count()
            .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_example_part1() {
        assert_eq!(Case::part1(Case::example().into()), 2);
    }

    #[test]
    fn check_part1() {
        assert_eq!(Case::part1(None), 526)
    }

    #[test]
    fn check_example_part2() {
        assert_eq!(Case::part2(Case::example().into()), 4);
    }

    #[test]
    fn check_part2() {
        assert_eq!(Case::part2(None), 886)
    }
}
