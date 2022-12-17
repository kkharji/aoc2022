use itertools::Itertools;

use super::{Case, GetInput};
#[derive(Debug, PartialEq)]
pub struct Input {
    pub grid: Vec<Vec<i8>>,
    pub dim: (i8, i8),
}

impl GetInput for Case {
    type Input = Input;

    const NAME: &'static str = "day08";

    fn text_to_input(content: &str) -> Self::Input {
        let grid = content
            .lines()
            .map(|v| v.chars().map(|c| c.to_digit(10).unwrap() as i8))
            .map(|v| v.collect_vec())
            .collect_vec();
        Input {
            dim: (grid[0].len() as i8, grid.len() as i8),
            grid,
        }
    }
}

#[test]
fn test_parse_example() {
    let output = Case::example();
    let expected = Input {
        dim: (5, 5),
        grid: vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ],
    };
    assert_eq!(output, expected)
}
