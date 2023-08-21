use std::convert::TryFrom;

use anyhow::{anyhow, Context};

use super::Input;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Source {
    Noop(Input),
    Not(Input),
    Or(Input, Input),
    And(Input, Input),
    ShiftLeft(Input, usize),
    ShiftRight(Input, usize),
}

impl TryFrom<&str> for Source {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut tokens = s.split_whitespace();

        let maybe_token = tokens.next();

        if let Some(token) = maybe_token {
            if token == "NOT" {
                return Ok(Source::Not(Input::try_from(
                    tokens.next().context("missing input for NOT gate")?,
                )?));
            }
        }

        if let Some(operator) = tokens.next() {
            match operator {
                "OR" => Ok(Source::Or(
                    Input::try_from(maybe_token.context("missing first input for OR gate")?)?,
                    Input::try_from(tokens.next().context("missing second input for OR gate")?)?,
                )),
                "AND" => Ok(Source::And(
                    Input::try_from(maybe_token.context("missing first input for AND gate")?)?,
                    Input::try_from(tokens.next().context("missing second input for AND gate")?)?,
                )),
                "LSHIFT" => Ok(Source::ShiftLeft(
                    Input::try_from(maybe_token.context("missing input for LSHIFT gate")?)?,
                    tokens
                        .next()
                        .context("missing offset for LSHIFT gate")?
                        .parse()?,
                )),
                "RSHIFT" => Ok(Source::ShiftRight(
                    Input::try_from(maybe_token.context("missing input for RSHIFT gate")?)?,
                    tokens
                        .next()
                        .context("missing offset for RSHIFT gate")?
                        .parse()?,
                )),
                _ => Err(anyhow!("invalid source: {:?}", s)),
            }
        } else {
            Ok(Source::Noop(Input::try_from(
                maybe_token.context("missing input")?,
            )?))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_from_str() {
        macro_rules! test {
            ($input:expr, $expected:expr) => {
                assert_eq!(Source::try_from($input).unwrap(), $expected);
            };
        }

        test!("123", Source::Noop(Input::Constant(123)));
        test!("NOT foo", Source::Not(Input::Wire("foo".to_owned())));
        test!(
            "123 AND foo",
            Source::And(Input::Constant(123), Input::Wire("foo".to_owned()))
        );
        test!(
            "foo OR 123",
            Source::Or(Input::Wire("foo".to_owned()), Input::Constant(123))
        );
        test!(
            "foo LSHIFT 1",
            Source::ShiftLeft(Input::Wire("foo".to_owned()), 1)
        );
        test!(
            "foo RSHIFT 2",
            Source::ShiftRight(Input::Wire("foo".to_owned()), 2)
        );
    }
}
