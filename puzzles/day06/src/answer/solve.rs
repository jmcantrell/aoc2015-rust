use crate::core::{Action, BoolGrid, UintGrid};

use super::{Parsed1, Parsed2};

type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

pub fn solve1(parsed: &Parsed1) -> anyhow::Result<Solution1> {
    let mut grid = BoolGrid::from_element(1000, 1000, false);

    for command in parsed {
        for location in command.rectangle.iter() {
            let value = &mut grid[(location.y, location.x)];

            *value = match command.action {
                Action::TurnOn => true,
                Action::TurnOff => false,
                Action::Toggle => !*value,
            };
        }
    }

    Ok(grid.into_iter().filter(|&&lit| lit).count())
}

pub fn solve2(parsed: &Parsed2) -> anyhow::Result<Solution2> {
    let mut grid = UintGrid::from_element(1000, 1000, 0);

    for command in parsed {
        for location in command.rectangle.iter() {
            let brightness = &mut grid[(location.y, location.x)];

            match command.action {
                Action::TurnOn => {
                    *brightness += 1;
                }
                Action::TurnOff => {
                    *brightness = brightness.saturating_sub(1);
                }
                Action::Toggle => {
                    *brightness += 2;
                }
            }
        }
    }

    Ok(grid.into_iter().sum())
}

#[cfg(test)]
pub mod tests {
    use crate::core::{Action, Command, Rectangle};

    use Action::*;

    #[test]
    fn solve1() {
        macro_rules! test {
            ($input:expr, $expected:expr) => {
                assert_eq!(
                    super::solve1(
                        &$input
                            .into_iter()
                            .map(|(action, top_left, bottom_right)| {
                                Command::new(
                                    action,
                                    Rectangle::new(top_left.into(), bottom_right.into()),
                                )
                            })
                            .collect::<Vec<_>>()
                    )
                    .unwrap(),
                    $expected
                );
            };
        }

        test!(
            [
                (TurnOn, [0, 0], [9, 9]),
                (TurnOff, [1, 1], [8, 8]),
                (Toggle, [2, 2], [7, 7])
            ],
            100 - 64 + 36
        );
    }

    #[test]
    fn solve2() {
        macro_rules! test {
            ($input:expr, $expected:expr) => {
                assert_eq!(
                    super::solve2(
                        &$input
                            .into_iter()
                            .map(|(action, top_left, bottom_right)| {
                                Command::new(
                                    action,
                                    Rectangle::new(top_left.into(), bottom_right.into()),
                                )
                            })
                            .collect::<Vec<_>>()
                    )
                    .unwrap(),
                    $expected
                );
            };
        }

        test!(
            [
                (TurnOn, [0, 0], [9, 9]),
                (TurnOn, [0, 0], [9, 9]),
                (TurnOff, [1, 1], [8, 8]),
                (Toggle, [2, 2], [7, 7])
            ],
            100 + 100 - 64 + 36 * 2
        );
    }
}
