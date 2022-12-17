mod input;

use crate::{day07::input::PrettyFormat, GetInput, Solve};

use self::input::Entry;

struct Case;

impl Solve for Case {
    fn part1(data: Option<Self::Input>) -> crate::Output {
        let root = data.unwrap_or_else(Self::data);
        let mut to_calculate = vec![root.clone()];
        let mut total = 0;
        while let Some(dir) = to_calculate.pop() {
            dir.borrow().children.values().for_each(|e| {
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

    fn part2(data: Option<Self::Input>) -> crate::Output {
        let root = data.unwrap_or_else(Self::data);
        let space_used = root.borrow().size();
        let space_available = 70_000_000 - space_used;
        let space_needed = 30_000_000 - space_available;
        let mut best = usize::MAX;

        println!("used space: {}", space_used.pretty_fmt());
        println!("free space: {}", space_available.pretty_fmt());
        println!("needed space: {}", space_needed.pretty_fmt());

        let mut to_visit = vec![root.clone()];

        while let Some(dir) = to_visit.pop() {
            dir.borrow().children.values().for_each(|e| {
                let Entry::Dir(dir) = e else { return };
                to_visit.push(dir.clone());
            });
            let size = dir.borrow().size();
            if size >= space_needed {
                best = best.min(size)
            }
        }

        println!("best space: {}", best.pretty_fmt());

        best.into()
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
    fn check_example_part2() {
        assert_eq!(Case::part2(Case::example().into()), 24933642);
    }

    #[test]
    fn check_part2() {
        assert_eq!(Case::part2(None), 7068748)
    }
}
