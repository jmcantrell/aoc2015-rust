use anyhow::Context;

use aoc::Input;

type Parsed = serde_json::Value;
pub type Parsed1 = Parsed;
pub type Parsed2 = Parsed;

fn parse(input: Input) -> anyhow::Result<Parsed> {
    serde_json::from_str(input).context("unable to parse input as json")
}

pub fn parse1(input: Input) -> anyhow::Result<Parsed1> {
    parse(input)
}

pub fn parse2(input: Input) -> anyhow::Result<Parsed2> {
    parse(input)
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse1() -> anyhow::Result<()> {
        macro_rules! test {
            ($input:expr) => {
                dbg!(super::parse1($input)?);
            };
        }

        test!("[1,2,3]");
        test!("{\"a\":2,\"b\":4}");
        test!("[[[3]]]");
        test!("{\"a\":{\"b\":4},\"c\":-1}");
        test!("{\"a\":[-1,1]}");
        test!("[-1,{\"a\":1}]");
        test!("[]");
        test!("{}");

        Ok(())
    }

    #[test]
    fn parse2() -> anyhow::Result<()> {
        macro_rules! test {
            ($input:expr) => {
                dbg!(super::parse2($input)?);
            };
        }

        test!("[1,2,3]");
        test!("[1,{\"c\":\"red\",\"b\":2},3]");
        test!("{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}");
        test!("[1,\"red\",5]");

        Ok(())
    }
}
