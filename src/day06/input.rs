use super::{Case, GetInput};

impl GetInput for Case {
    type Input = String;

    const NAME: &'static str = "day06";

    fn text_to_input(content: &str) -> Self::Input {
        content.to_string()
    }
}

#[test]
#[ignore = "no pre-process for input and no valid example"]
fn test_parse_example() {
    let output = Case::example();
    assert_eq!(output, String::default())
}
