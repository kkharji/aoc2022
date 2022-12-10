use std::collections::VecDeque;

use super::Case;
use crate::GetInput;

impl GetInput for Case {
    type Input = Vec<(char, char)>;

    const NAME: &'static str = "day02";

    fn text_to_input(content: &str) -> Self::Input {
        content
            .split("\n")
            .flat_map(|line| {
                let mut chars = line
                    .chars()
                    .filter(|w| !w.is_whitespace())
                    .collect::<VecDeque<char>>();

                Some((chars.pop_front()?, chars.pop_back()?))
            })
            .collect()
    }
}

#[test]
fn test_parse_example() {
    let output = Case::example();
    println!("{output:#?}");
    assert_eq!(output, vec![('A', 'Y'), ('B', 'X'), ('C', 'Z')])
}
