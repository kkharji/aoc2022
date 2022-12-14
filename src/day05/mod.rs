mod input;

use crate::{GetInput, Solve};

struct Case;

impl Solve for Case {
    fn part1(data: Option<Self::Input>) -> crate::Output {
        let mut data = data.unwrap_or_else(Self::data);

        for (q, f, t) in data.procedures.iter() {
            for _ in 0..*q {
                let item = data.stacks[f - 1].pop().unwrap();
                data.stacks[t - 1].push(item);
            }
        }

        data.join_stacks_last_items()
    }

    fn part2(data: Option<Self::Input>) -> crate::Output {
        let mut data = data.unwrap_or_else(Self::data);

        for (q, f, t) in data.procedures.iter() {
            let (t, f) = (t - 1, f - 1);
            let len = data.stacks[t].len() + q;

            data.stacks[t].resize(len, ' ');

            for i in 0..*q {
                data.stacks[t][len - 1 - i] = data.stacks[f].pop().unwrap();
            }
        }

        data.join_stacks_last_items()
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

    fn check_example_part2() {
        assert_eq!(Case::part2(Case::example().into()), "MCD");
    }

    #[test]
    fn check_part2() {
        assert_eq!(Case::part2(None), "VRZGHDFBQ")
    }
}
