use std::convert::TryFrom;

use anyhow::Context;
use lazy_regex::{lazy_regex, Lazy, Regex};

#[derive(Debug, Clone)]
pub struct Reindeer<'a> {
    pub name: &'a str,
    pub top_speed: usize,
    pub fly_duration: usize,
    pub rest_duration: usize,
}

impl<'a> Reindeer<'a> {
    pub fn new(name: &'a str, top_speed: usize, fly_duration: usize, rest_duration: usize) -> Self {
        Self {
            name,
            top_speed,
            fly_duration,
            rest_duration,
        }
    }

    pub fn distance_traveled_after(&self, total_duration: usize) -> usize {
        let period_duration = self.fly_duration + self.rest_duration;

        let whole_periods = total_duration / period_duration;
        let remaining_seconds = total_duration % period_duration;

        self.top_speed
            * (whole_periods * self.fly_duration + self.fly_duration.min(remaining_seconds))
    }
}

static RE: Lazy<Regex> = lazy_regex!(
    r"^(?<name>\S+) can fly (?<top_speed>\d+) km/s for (?<fly_duration>\d+) seconds, but then must rest for (?<rest_duration>\d+) seconds\."
);

impl<'a> TryFrom<&'a str> for Reindeer<'a> {
    type Error = anyhow::Error;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let captures = RE
            .captures(s)
            .with_context(|| format!("expected input to match: {:?}", RE.as_str()))?;

        let name = captures.name("name").unwrap().as_str();
        let top_speed = captures["top_speed"].parse()?;
        let fly_duration = captures["fly_duration"].parse()?;
        let rest_duration = captures["rest_duration"].parse()?;

        Ok(Self {
            name,
            top_speed,
            fly_duration,
            rest_duration,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance_traveled_after() {
        macro_rules! test {
            ($name:expr, $speed:expr, $fly:expr, $rest:expr, $duration:expr, $expected:expr) => {
                assert_eq!(
                    Reindeer::new($name, $speed, $fly, $rest).distance_traveled_after($duration),
                    $expected
                );
            };
        }

        test!("Comet", 14, 10, 127, 1000, 1120);
        test!("Dancer", 16, 11, 162, 1000, 1056);
    }
}
