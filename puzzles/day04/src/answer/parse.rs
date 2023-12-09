use aoc::Input;

type Parsed = Vec<u8>;
pub type Parsed1 = Parsed;
pub type Parsed2 = Parsed;

fn parse(input: Input) -> anyhow::Result<Parsed> {
    Ok(input.trim().as_bytes().to_vec())
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

        test!("", []);
        test!("abcdef", [97, 98, 99, 100, 101, 102]);
        test!("abcdef\n", [97, 98, 99, 100, 101, 102]);
    }
}
