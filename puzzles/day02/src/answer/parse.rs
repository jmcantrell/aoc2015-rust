use anyhow::Context;

use aoc::Input;

use crate::core::Present;

type Parsed = Vec<Present>;
pub type Parsed1 = Parsed;
pub type Parsed2 = Parsed;

fn parse(input: Input) -> anyhow::Result<Parsed> {
    input
        .lines()
        .enumerate()
        .map(|(i, s)| {
            s.try_into()
                .with_context(|| format!("present number {}", i + 1))
        })
        .collect()
}

pub fn parse1(input: Input) -> anyhow::Result<Parsed1> {
    parse(input)
}

pub fn parse2(input: Input) -> anyhow::Result<Parsed2> {
    parse(input)
}

#[cfg(test)]
mod tests {
    use crate::core::Present;

    #[test]
    fn parse() {
        macro_rules! test {
            ($input:expr, $expected:expr) => {
                assert_eq!(
                    super::parse($input).unwrap(),
                    $expected.into_iter().map(Present::from).collect::<Vec<_>>()
                );
            };
        }

        test!("2x3x4\n1x1x10\n", [[2, 3, 4], [1, 1, 10]]);
    }
}
