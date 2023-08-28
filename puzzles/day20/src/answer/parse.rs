use aoc::Input;

type Parsed = usize;
pub type Parsed1 = Parsed;
pub type Parsed2 = Parsed;

fn parse(input: Input) -> anyhow::Result<Parsed> {
    Ok(input.trim().parse()?)
}

pub fn parse1(input: Input) -> anyhow::Result<Parsed1> {
    parse(input)
}

pub fn parse2(input: Input) -> anyhow::Result<Parsed2> {
    parse(input)
}

#[cfg(test)]
mod tests {
    use aoc::Input;

    const INPUT: Input = include_str!("../../input.txt");

    #[test]
    fn parse() -> anyhow::Result<()> {
        dbg!(super::parse(INPUT)?);
        Ok(())
    }
}
