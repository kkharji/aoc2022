use std::ops::Div;

use super::{Case, GetInput};

impl GetInput for Case {
    type Input = Vec<(String, String)>;

    const NAME: &'static str = "day03";

    fn text_to_input(content: &str) -> Self::Input {
        content
            .split("\n")
            .filter(|line| !line.is_empty())
            .map(|line| line.split_at(line.len().div(2)))
            .map(|(a, b)| (a.to_string(), b.to_string()))
            .collect()
    }
}

#[test]
fn test_parse_example() {
    let output = Case::example();
    let expected = [
        ("vJrwpWtwJgWr", "hcsFMMfFFhFp"),
        ("jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL"),
        ("PmmdzqPrV", "vPwwTWBwg"),
        ("wMqvLMZHhHMvwLH", "jbvcjnnSBnvTQFn"),
        ("ttgJtRGJ", "QctTZtZT"),
        ("CrZsJsPPZsGz", "wwsLwLmpwMDw"),
    ]
    .map(|(a, b)| (a.to_string(), b.to_string()))
    .to_vec();
    assert_eq!(output, expected)
}
