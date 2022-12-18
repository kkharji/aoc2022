use super::{Case, GetInput};
use itertools::Itertools;
use std::str::FromStr;
use strum::EnumString;

pub type Int = isize;

#[derive(EnumString, PartialEq, Debug, Clone, Copy)]
pub enum Direction {
    #[strum(serialize = "U")]
    Up,
    #[strum(serialize = "D")]
    Down,
    #[strum(serialize = "L")]
    Left,
    #[strum(serialize = "R")]
    Right,
}

impl GetInput for Case {
    type Input = Vec<(Direction, Int)>;

    const NAME: &'static str = "day09";

    fn text_to_input(content: &str) -> Self::Input {
        content
            .lines()
            .flat_map(|l| l.split_once(" "))
            .flat_map(|(d, n)| {
                let amount = n.parse::<Int>().ok()?;
                let direction = Direction::from_str(d).ok()?;
                Some((direction, amount))
            })
            .collect_vec()
    }
}

impl Direction {
    const DIR: [(Int, Int); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    pub fn to_point(&self) -> (Int, Int) {
        Self::DIR[*self as usize]
    }
}

#[test]
fn test_parse_example() {
    use Direction::*;
    let output = Case::example();
    let motions = vec![
        (Right, 4),
        (Up, 4),
        (Left, 3),
        (Down, 1),
        (Right, 4),
        (Down, 1),
        (Left, 5),
        (Right, 2),
    ];
    println!("{output:#?}");
    assert_eq!(output, motions)
}
