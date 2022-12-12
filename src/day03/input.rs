use super::{Case, GetInput};

impl GetInput for Case {
    type Input = Vec<Vec<char>>;

    const NAME: &'static str = "day03";

    fn text_to_input(content: &str) -> Self::Input {
        content
            .split("\n")
            .filter(|line| !line.is_empty())
            .map(|a| a.chars().collect())
            .collect()
    }
}

#[test]
fn test_parse_example() {
    let output = Case::example();
    let expected = [
        "vJrwpWtwJgWrhcsFMMfFFhFp",
        "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
        "PmmdzqPrVvPwwTWBwg",
        "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
        "ttgJtRGJQctTZtZT",
        "CrZsJsPPZsGzwwsLwLmpwMDw",
    ]
    .map(|a| a.chars().collect::<Vec<_>>())
    .to_vec();
    assert_eq!(output, expected)
}
