use std::convert::TryFrom;

use super::Signal;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Input {
    Constant(Signal),
    Wire(String),
}

impl TryFrom<&str> for Input {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if let Ok(signal) = s.parse::<Signal>() {
            Ok(Self::Constant(signal))
        } else {
            Ok(Self::Wire(s.to_owned()))
        }
    }
}
