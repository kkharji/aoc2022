mod input;

use crate::{GetInput, Solve};

use self::input::Entry;

struct Case;

impl Solve for Case {
    fn part1(data: Option<Self::Input>) -> crate::Output {
        let root = data.unwrap_or_else(Self::data);
        let mut to_calculate = vec![root.clone()];
        let mut total = 0;
        while let Some(dir) = to_calculate.pop() {
            dir.borrow().childern.values().for_each(|e| {
                let Entry::Dir(dir) = e else { return };
                to_calculate.push(dir.clone());
            });
            let size = dir.borrow().size();
            if size <= 100000 {
                total += size;
            }
        }
        total.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_example_part1() {
        assert_eq!(Case::part1(Case::example().into()), 95437);
    }

    #[test]
    fn check_part1() {
        assert_eq!(Case::part1(None), 1307902)
    }

    #[test]
    #[ignore = "no-implemented"]
    fn check_example_part2() {
        assert_eq!(Case::part2(Case::example().into()), 24000);
    }

    #[test]
    #[ignore = "no-implemented"]
    fn check_part2() {
        assert_eq!(Case::part2(None), 205381)
    }
}
