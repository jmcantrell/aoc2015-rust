use aoc::Input;

type Parsed = Vec<&'static str>;
pub type Parsed1 = Parsed;
pub type Parsed2 = Parsed;

fn parse(input: Input) -> anyhow::Result<Parsed> {
    Ok(input.lines().collect())
}

pub fn parse1(input: Input) -> anyhow::Result<Parsed1> {
    parse(input)
}

pub fn parse2(input: Input) -> anyhow::Result<Parsed2> {
    parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        macro_rules! test {
            ($input:expr, $expected:expr) => {
                assert_eq!(parse($input).unwrap(), Vec::from($expected));
            };
        }

        test!("abc\ndef\n", ["abc", "def"]);
    }
}
