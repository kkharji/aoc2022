mod input;

use crate::{GetInput, Solve};

struct Case;

impl Solve for Case {
    fn part1(data: Option<Self::Input>) -> crate::Output {
        data.unwrap_or_else(Self::data)
            .into_iter()
            .filter(|sap| {
                let lhs: Vec<_> = (sap.lhs.start..sap.lhs.end + 1).collect();
                let rhs: Vec<_> = (sap.rhs.start..sap.rhs.end + 1).collect();
                let rhslg = rhs.len() > lhs.len();
                let (lg, sh) = if rhslg { (rhs, lhs) } else { (lhs, rhs) };
                sh.into_iter().all(|i| lg.contains(&i))
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
