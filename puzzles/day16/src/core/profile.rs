use std::collections::HashMap;

use anyhow::Context;

use super::Compound;

pub type Inner = HashMap<Compound, usize>;

#[derive(Debug, Clone)]
pub struct Profile(Inner);

impl Profile {
    pub fn matches(&self, other: &Self) -> bool {
        self.matches_with(other, |_, value, other_value| value == other_value)
    }

    pub fn matches_with<F>(&self, other: &Self, test: F) -> bool
    where
        F: Fn(&Compound, &usize, &usize) -> bool,
    {
        self.0.iter().all(|(key, value)| {
            other
                .0
                .get(key)
                .is_some_and(|other_value| test(key, value, other_value))
        })
    }
}

impl TryFrom<&str> for Profile {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        fn parse_mapping(s: &str) -> anyhow::Result<(Compound, usize)> {
            let (key, value) = s
                .split_once(':')
                .context("expected compound and amount to be delimited by a colon")?;

            let compound = key.trim().try_into()?;
            let amount = value.trim().parse()?;

            Ok((compound, amount))
        }

        let inner = s
            .split(',')
            .enumerate()
            .map(|(i, s)| parse_mapping(s).with_context(|| format!("mapping number {}", i + 1)))
            .collect::<Result<HashMap<_, _>, _>>()?;

        Ok(Self(inner))
    }
}
