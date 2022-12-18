mod input;

use crate::{GetInput, Solve};
use std::collections::HashSet;

use self::input::Int;

struct Case;

impl Solve for Case {
    fn part1(data: Option<Self::Input>) -> crate::Output {
        let [mut head, mut tail]: [(Int, Int); 2] = Default::default();
        let mut seen: HashSet<(Int, Int)> = Default::default();
        let motions = data.unwrap_or_else(Self::data);

        for (dir, amount) in motions {
            let delta = dir.to_point();
            for _ in 0..amount {
                head.0 += delta.0;
                head.1 += delta.1;

                let (r, c) = (head.0 - tail.0, head.1 - tail.1);
                let (r_abs, c_abs) = (r.abs(), c.abs());

                if r == 0 && c_abs > 1 {
                    tail.1 += c.signum();
                } else if c == 0 && r_abs > 1 {
                    tail.0 += r.signum();
                } else if r_abs > 1 || c_abs > 1 {
                    tail.0 += r.signum();
                    tail.1 += c.signum();
                }

                seen.insert(tail.clone());
            }
        }

        seen.len().into()
    }
}

#[test]
fn part1_example() {
    assert_eq!(Case::part1(Case::example().into()), 13);
}

#[test]
fn part1() {
    assert_eq!(Case::part1(None), 6209)
}

#[test]
#[ignore = "not implemented"]
fn part2_example() {
    assert_eq!(Case::part2(Case::example().into()), 24000);
}

#[test]
#[ignore = "not implemented"]
fn part2() {
    assert_eq!(Case::part2(None), 205381)
}
