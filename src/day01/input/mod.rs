use super::Day01;
use crate::GetInput;

impl GetInput for Day01 {
    type Input = super::Input;

    fn text_to_input(content: &str) -> Self::Input {
        todo!()
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
}
