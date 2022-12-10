use super::Case;
use crate::GetInput;

impl GetInput for Case {
    type Input = Vec<Vec<i32>>;

    fn text_to_input(content: &str) -> Self::Input {
        let mut elfs: Self::Input = vec![];
        let mut lines = content.split("\n");
        let mut current_elf = 0;

        while let Some(cal) = lines.next() {
            if cal.is_empty() {
                current_elf += 1;
                continue;
            }
            let val = cal.parse().expect("number found something else");
            if let Some(elem) = elfs.get_mut(current_elf) {
                elem.push(val)
            } else {
                elfs.push(vec![val])
            }
        }
        elfs
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
    assert_eq!(output[1], vec![4000])
}
