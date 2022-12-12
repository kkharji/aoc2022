use super::{Case, GetInput};

impl GetInput for Case {
    type Input = ();

    const NAME: &'static str = "day03";

    fn text_to_input(_content: &str) -> Self::Input {
        ()
    }
}

#[test]
fn test_parse_example() {
    let output = Case::example();
    println!("{output:#?}");
    assert_eq!(output, ())
}
