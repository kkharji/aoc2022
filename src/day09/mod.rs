mod input;

use crate::{GetInput, Solve};
use std::collections::HashSet;

use self::input::Int;

struct Case;

impl Solve for Case {
    fn part1(data: Option<Self::Input>) -> crate::Output {
        let mut rope = Rope::new(2);

        data.unwrap_or_else(Self::data)
            .into_iter()
            .for_each(|(dir, amount)| rope.process(dir.to_point(), amount));

        rope.seen.len().into()
    }

    fn part2(data: Option<Self::Input>) -> crate::Output {
        let mut rope = Rope::new(10);

        data.unwrap_or_else(Self::data)
            .into_iter()
            .for_each(|(dir, amount)| rope.process(dir.to_point(), amount));

        rope.seen.len().into()
    }
}

#[derive(Debug, Default)]
pub struct Rope {
    pub segs: Vec<(Int, Int)>,
    pub seen: HashSet<(Int, Int)>,
}

impl Rope {
    pub fn new(len: usize) -> Self {
        Self {
            segs: vec![(0, 0); len],
            ..Default::default()
        }
    }

    pub fn process(&mut self, delta: (Int, Int), amount: Int) {
        for _ in 0..amount {
            self.segs[0].0 += delta.0;
            self.segs[0].1 += delta.1;

            for i in 1..self.segs.len() {
                let r = self.segs[i - 1].0 - self.segs[i].0;
                let c = self.segs[i - 1].1 - self.segs[i].1;
                let (r_abs, c_abs) = (r.abs(), c.abs());

                if r == 0 && c_abs > 1 {
                    self.segs[i].1 += c.signum();
                } else if c == 0 && r_abs > 1 {
                    self.segs[i].0 += r.signum();
                } else if r_abs > 1 || c_abs > 1 {
                    self.segs[i].0 += r.signum();
                    self.segs[i].1 += c.signum();
                }
            }

            self.seen.insert(self.segs[self.segs.len() - 1].clone());
        }
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
fn part2_example() {
    use tap::Pipe;
    "
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"
        .pipe(|text| Case::text_to_input(text))
        .pipe(|input| assert_eq!(Case::part2(input.into()), 36));
}

#[test]
fn part2() {
    assert_eq!(Case::part2(None), 2460)
}
