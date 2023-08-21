use std::convert::TryFrom;

use anyhow::anyhow;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}

impl TryFrom<&str> for Action {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "turn on" => Ok(Self::TurnOn),
            "turn off" => Ok(Self::TurnOff),
            "toggle" => Ok(Self::Toggle),
            _ => Err(anyhow!("invalid action: {:?}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use Action::*;

    #[test]
    fn try_from_str() {
        macro_rules! test {
            ($input:expr, $expected:expr) => {
                assert_eq!(Action::try_from($input).unwrap(), $expected);
            };
        }

        test!("turn on", TurnOn);
        test!("turn off", TurnOff);
        test!("toggle", Toggle);

        assert!(Action::try_from("bogus").is_err());
    }
}
