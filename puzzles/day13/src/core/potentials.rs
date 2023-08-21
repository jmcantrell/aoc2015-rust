use std::collections::HashMap;
use std::convert::TryFrom;
use std::iter::once;

use anyhow::Context;
use itertools::Itertools;
use lazy_regex::{lazy_regex, Lazy, Regex};

use super::Happiness;

type Inner<'a> = HashMap<&'a str, HashMap<&'a str, Happiness>>;

#[derive(Debug, Clone)]
pub struct Potentials<'a>(Inner<'a>);

impl Potentials<'_> {
    pub fn add_me(&mut self) {
        // Since empty names are not allowed in the input,
        // representing myself as the empty string seems safe from collision.
        self.0.entry("").or_default();
    }

    pub fn optimal_happiness(&self) -> Happiness {
        self.0
            .keys()
            .permutations(self.0.len())
            .map(|arrangement| {
                once(arrangement.last().unwrap())
                    .chain(arrangement.iter())
                    .tuple_windows::<(_, _)>()
                    .map(|(&&a, &&b)| {
                        self.0[a].get(b).cloned().unwrap_or_default()
                            + self.0[b].get(a).cloned().unwrap_or_default()
                    })
                    .sum()
            })
            .max()
            .unwrap_or_default()
    }
}

static RE: Lazy<Regex> = lazy_regex!(
    r"^(?<person>\S+) would (?<sign>gain|lose) (?<change>\d+) happiness units by sitting next to (?<neighbor>\S+)\.$"
);

impl<'a> TryFrom<&'a str> for Potentials<'a> {
    type Error = anyhow::Error;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        fn parse_neighbors(s: &str) -> anyhow::Result<(&str, Happiness, &str)> {
            let captures = RE
                .captures(s)
                .context(format!("expected line to match: {:?}", RE.to_string()))?;

            let person = captures.name("person").unwrap().as_str();
            let sign = if &captures["sign"] == "gain" { 1 } else { -1 };
            let change: isize = captures["change"].parse()?;
            let neighbor = captures.name("neighbor").unwrap().as_str();

            Ok((person, sign * change, neighbor))
        }

        let mut inner: Inner = HashMap::new();

        for (i, s) in s.lines().enumerate() {
            let (person, change, neighbor) =
                parse_neighbors(s).context(format!("line number {}", i + 1))?;
            inner.entry(person).or_default().insert(neighbor, change);
        }

        Ok(Self(inner))
    }
}
