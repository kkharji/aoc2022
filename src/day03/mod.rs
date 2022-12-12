mod input;

use crate::{GetInput, Solve};

struct Case;

impl Solve for Case {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "no-implemented"]
    fn check_example_part1() {
        assert_eq!(Case::part1(Case::example().into()), 24000);
    }

    #[test]
    #[ignore = "no-implemented"]
    fn check_example_part2() {
        assert_eq!(Case::part2(Case::example().into()), 24000);
    }

    #[test]
    #[ignore = "no-implemented"]
    fn check_part1() {
        assert_eq!(Case::part1(None), 70296)
    }

    #[test]
    #[ignore = "no-implemented"]
    fn check_part2() {
        assert_eq!(Case::part2(None), 205381)
    }
}
