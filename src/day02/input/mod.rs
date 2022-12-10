use std::collections::VecDeque;

use super::Case;
use crate::GetInput;

impl GetInput for Case {
    type Input = Vec<(char, char)>;

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

    fn data() -> Self::Input {
        Self::text_to_input(include_str!("./data.txt"))
    }

    fn example() -> Self::Input {
        Self::text_to_input(include_str!("./example.txt"))
    }
}

#[test]
fn test_parse_example() {
    let output = Case::example();
    println!("{output:#?}");
    assert_eq!(output, vec![('A', 'Y'), ('B', 'X'), ('C', 'Z')])
}
