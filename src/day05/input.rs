use super::{Case, GetInput};
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Input {
    pub procedures: Vec<(usize, usize, usize)>,
    pub stacks: Vec<Vec<char>>,
}

impl GetInput for Case {
    type Input = Input;

    const NAME: &'static str = "day05";

    fn text_to_input(content: &str) -> Self::Input {
        let (boxes, rest) = content.split_once("\n\n").unwrap();
        let mut stacks = vec![vec![]; 9];
        let procedures = rest
            .lines()
            .map(|l| {
                l.split_whitespace()
                    .filter_map(|s| s.parse::<usize>().ok())
                    .collect_tuple()
                    .unwrap()
            })
            .collect::<Vec<_>>();

        // Extract stacks
        for v in boxes.lines().rev() {
            for (idx, chars) in v.chars().collect_vec().chunks(4).enumerate() {
                for c in chars.iter().filter(|v| v.is_alphabetic()) {
                    stacks[idx].push(*c);
                }
            }
        }

        stacks.retain(|vec| !vec.is_empty());

        Input { procedures, stacks }
    }
}

#[test]
fn test_parse_example() {
    let output = Case::example();
    let procedures = [(1, 2, 1), (3, 1, 3), (2, 2, 1), (1, 1, 2)].into();
    let stacks = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
    let expected = Input { procedures, stacks };

    assert_eq!(output, expected)
}
