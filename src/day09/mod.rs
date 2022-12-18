mod input;

use crate::{GetInput, Solve};

struct Case;

impl Solve for Case {
    fn part1(data: Option<Self::Input>) -> crate::Output {
        todo!()
    }
    
}

#[test]
fn part1_example() {
    assert_eq!(Case::part1(Case::example().into()), 13);
}

#[test]
#[ignore = "not implemented"]
fn part1() {
    assert_eq!(Case::part1(None), 70296)
}

#[test]
#[ignore = "not implemented"]
fn part2_example() {
    assert_eq!(Case::part2(Case::example().into()), 24000);
}

#[test]
#[ignore = "not implemented"]
fn part2() {
    assert_eq!(Case::part2(None), 205381)
}
