use std::collections::HashSet;

use anyhow::Context;

use crate::core::Password;

use super::{Parsed1, Parsed2};

type Solution = String;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

fn iter_valid_passwords(start: &Password) -> impl Iterator<Item = Password> {
    let letters: Vec<_> = ('a'..='z').collect();
    let straights: HashSet<_> = letters.windows(3).map(|window| window.to_vec()).collect();
    let confusing: HashSet<_> = ['i', 'o', 'l'].into_iter().collect();
    let pairs: Vec<_> = letters.iter().map(|&c| [c, c]).collect();

    let mut passwords = start.iter();

    std::iter::from_fn(move || loop {
        let password = passwords.next().unwrap();

        if let Some(i) = password.chars().position(|c| confusing.contains(c)) {
            passwords.skip_letter_at(i);
            continue;
        }

        if !password
            .as_ref()
            .windows(3)
            .any(|window| straights.contains(window))
        {
            continue;
        }

        if pairs
            .iter()
            .filter(|&pair| password.as_ref().windows(2).any(|window| window == pair))
            .take(2)
            .count()
            < 2
        {
            continue;
        }

        break Some(password);
    })
}

pub fn solve1(parsed: &Parsed1) -> anyhow::Result<Solution1> {
    iter_valid_passwords(parsed)
        .next()
        .map(|password| password.to_string())
        .context("no password")
}

pub fn solve2(parsed: &Parsed2) -> anyhow::Result<Solution2> {
    iter_valid_passwords(parsed)
        .nth(1)
        .map(|password| password.to_string())
        .context("no password")
}

#[cfg(test)]
mod tests {
    use crate::core::Password;

    use super::*;

    #[test]
    fn test_iter_valid_passwords() {
        macro_rules! test {
            ($input:expr, $expected:expr) => {
                assert_eq!(
                    iter_valid_passwords(&Password::try_from($input).unwrap())
                        .next()
                        .unwrap()
                        .to_string(),
                    $expected
                );
            };
        }

        test!("abcdefgh", "abcdffaa");
        test!("ghijklmn", "ghjaabcc");
    }
}
