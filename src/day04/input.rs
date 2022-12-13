use itertools::Itertools;
use tap::Pipe;

use super::{Case, GetInput};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SectionRange {
    pub start: usize,
    pub end: usize,
}

impl From<(usize, usize)> for SectionRange {
    fn from((start, end): (usize, usize)) -> Self {
        SectionRange { start, end }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SectionAssignmentPair {
    pub lhs: SectionRange,
    pub rhs: SectionRange,
}

fn parse_pair(str: &str) -> Option<(usize, usize)> {
    str.split("-")
        .flat_map(|v| v.parse::<usize>())
        .collect_tuple()
}

impl GetInput for Case {
    type Input = Vec<SectionAssignmentPair>;

    const NAME: &'static str = "day04";

    fn text_to_input(content: &str) -> Self::Input {
        content
            .split("\n")
            .flat_map(|line| line.split(",").collect_tuple())
            .flat_map(|(lhs, rhs)| {
                SectionAssignmentPair {
                    lhs: parse_pair(lhs)?.into(),
                    rhs: parse_pair(rhs)?.into(),
                }
                .pipe(Some)
            })
            .collect()
    }
}

#[test]
fn test_parse_example() {
    let output = Case::example();
    let expected = [
        ((2, 4), (6, 8)),
        ((2, 3), (4, 5)),
        ((5, 7), (7, 9)),
        ((2, 8), (3, 7)),
        ((6, 6), (4, 6)),
        ((2, 6), (4, 8)),
    ]
    .map(|(lhs, rhs)| SectionAssignmentPair {
        lhs: lhs.into(),
        rhs: rhs.into(),
    })
    .to_vec();
    assert_eq!(output, expected)
}
