mod input;

use crate::{GetInput, Solve};

struct Case;

impl Solve for Case {
    fn part1(data: Option<Self::Input>) -> crate::Output {
        let mut data = data.unwrap_or_else(Self::data);

        for (q, f, t) in data.procedures {
            for _ in 0..q {
                let item = data.stacks[f - 1].pop().unwrap();
                data.stacks[t - 1].push(item);
            }
        }

        data.stacks
            .iter()
            .flat_map(|s| s.last())
            .collect::<String>()
            .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_example_part1() {
        assert_eq!(Case::part1(Case::example().into()), "CMZ");
    }

    #[test]
    fn check_part1() {
        assert_eq!(Case::part1(None), "SHMSDGZVC")
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
