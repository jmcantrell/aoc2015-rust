use std::convert::TryFrom;

use anyhow::{ensure, Context};

use super::{Action, Location, Rectangle};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Command {
    pub action: Action,
    pub rectangle: Rectangle,
}

impl Command {
    pub fn new(action: Action, rectangle: Rectangle) -> Self {
        Self { action, rectangle }
    }
}

impl TryFrom<&str> for Command {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        fn parse_location(s: &str) -> anyhow::Result<Location> {
            let mut tokens = s.split(',');

            let row: usize = tokens.next().context("missing row")?.parse()?;
            let column: usize = tokens.next().context("missing column")?.parse()?;

            ensure!(tokens.next().is_none(), "unexpected trailing characters",);

            Ok(Location::from([column, row]))
        }

        fn rsplit_once_whitespace(s: &str) -> Option<(&str, &str)> {
            s.rsplit_once(|c: char| c.is_whitespace())
        }

        // Gobbling tokens from the end because the action possibly have spaces and the remaining
        // tokens do not, so it's easier to parse those first and treat the remainder as the
        // action name.

        let (s, token) = rsplit_once_whitespace(s).context("missing range end")?;

        let bottom_right = parse_location(token).context("invalid range end")?;

        let s = s
            .strip_suffix(" through")
            .context("expected ' through ' to delimit range bounds")?;

        let (s, token) = rsplit_once_whitespace(s).context("missing range start")?;

        let top_left = parse_location(token).context("invalid range start")?;

        let action = Action::try_from(s)?;
        let rectangle = Rectangle::new(top_left, bottom_right);

        Ok(Self { action, rectangle })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use Action::*;

    #[test]
    fn try_from_str() {
        macro_rules! test {
            ($input:expr, $action:expr, $top_left:expr, $bottom_right:expr) => {
                assert_eq!(
                    Command::try_from($input).unwrap(),
                    Command::new(
                        $action,
                        Rectangle::new($top_left.into(), $bottom_right.into())
                    )
                );
            };
        }

        test!("turn on 0,1 through 2,3", TurnOn, [1, 0], [3, 2]);
        test!("turn off 1,2 through 3,4", TurnOff, [2, 1], [4, 3]);
        test!("toggle 2,3 through 4,5", Toggle, [3, 2], [5, 4]);
    }
}
