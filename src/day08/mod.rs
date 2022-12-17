mod input;

use std::collections::HashSet;

use itertools::fold;
use tap::Pipe;

use crate::{GetInput, Solve};

use self::input::Input;

struct Case;

static U: (i8, i8) = (-1, 0);
static D: (i8, i8) = (1, 0);
static L: (i8, i8) = (0, -1);
static R: (i8, i8) = (0, 1);

impl Solve for Case {
    fn part1(data: Option<Self::Input>) -> crate::Output {
        let Input { grid, dim: (h, w) } = data.unwrap_or_else(Self::data);
        let r_d = ((0, 0), R, D);
        let d_r = ((0, 0), D, R);
        let u_l = ((h - 1, w - 1), U, L);
        let l_u = ((h - 1, w - 1), L, U);

        [r_d, d_r, u_l, l_u]
            .iter()
            .fold(HashSet::new(), |mut acc, (mut walk, step, search)| {
                while (walk.0 >= 0 && walk.1 >= 0) && (walk.0 < h && walk.1 < w) {
                    let (mut row, mut col) = walk;
                    let mut tallest = grid[row as usize][col as usize];

                    acc.insert((row, col));

                    while tallest < 9 {
                        row += search.0;
                        col += search.1;

                        if (row < 0 || col < 0) || (row >= h || col >= w) {
                            break;
                        }

                        grid[row as usize][col as usize].pipe(|tree| {
                            (tree > tallest).then(|| {
                                acc.insert((row, col));
                                tallest = tree;
                            })
                        });
                    }

                    walk.0 += step.0;
                    walk.1 += step.1;
                }

                acc
            })
            .len()
            .into()
    }

    fn part2(data: Option<Self::Input>) -> crate::Output {
        let Input { grid, dim: (h, w) } = data.unwrap_or_else(Self::data);
        let steps = [U, D, L, R];

        (1..h - 1)
            .into_iter()
            .flat_map(|r| {
                (1..w - 1)
                    .map({
                        |c| {
                            steps.map(|step| {
                                ((r + step.0, c + step.1), step, grid[r as usize][c as usize])
                            })
                        }
                    })
                    .map({
                        |iterator| {
                            fold(iterator, 1, |score, ((mut row, mut col), step, height)| {
                                let mut count = 0;

                                while (row >= 0 && col >= 0) && (row < h && col < w) {
                                    count += 1;

                                    if grid[row as usize][col as usize] >= height {
                                        break;
                                    }

                                    row += step.0;
                                    col += step.1
                                }

                                score * count
                            })
                        }
                    })
                    .max()
            })
            .max()
            .unwrap_or_default()
            .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_example_part1() {
        assert_eq!(Case::part1(Case::example().into()), 21);
    }

    #[test]
    fn check_part1() {
        assert_eq!(Case::part1(None), 1843)
    }

    #[test]
    fn check_example_part2() {
        assert_eq!(Case::part2(Case::example().into()), 8);
    }

    #[test]
    fn check_part2() {
        assert_eq!(Case::part2(None), 180000)
    }
}


