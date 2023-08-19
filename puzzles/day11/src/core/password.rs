use anyhow::{bail, ensure};

use std::convert::TryFrom;

const PASSWORD_LEN: usize = 8;

type Inner = Vec<char>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Password(Inner);

impl Password {
    pub fn iter(&self) -> PasswordIter {
        PasswordIter::after(self.clone())
    }

    pub fn chars(&self) -> impl Iterator<Item = &char> {
        self.0.iter()
    }
}

impl From<Password> for Inner {
    fn from(password: Password) -> Self {
        password.0
    }
}

impl AsRef<Inner> for Password {
    fn as_ref(&self) -> &Inner {
        &self.0
    }
}

impl std::fmt::Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for c in self.0.iter() {
            write!(f, "{}", c)?;
        }
        Ok(())
    }
}

impl TryFrom<&str> for Password {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        ensure!(
            s.len() == PASSWORD_LEN,
            "expected password to be {} characters, but it was {}",
            PASSWORD_LEN,
            s.len()
        );

        let chars: Vec<_> = s.chars().collect();

        for (i, c) in chars.iter().enumerate() {
            if !(c.is_alphabetic() && c.is_lowercase()) {
                bail!(
                    "character number {} is not a lowercase letter: {:?}",
                    i + 1,
                    c
                );
            }
        }

        Ok(Self(chars))
    }
}

#[derive(Debug, Clone)]
pub struct PasswordIter(Inner);

fn next_char(c: char) -> (char, bool) {
    if c == 'z' {
        ('a', true)
    } else {
        (char::from_u32(c as u32 + 1).unwrap(), false)
    }
}

impl PasswordIter {
    pub fn after(password: Password) -> Self {
        Self(password.into())
    }

    pub fn advance_index(&mut self, i: usize) {
        for j in (i + 1)..self.0.len() {
            self.0[j] = 'z';
        }
    }
}

impl Iterator for PasswordIter {
    type Item = Password;

    fn next(&mut self) -> Option<Self::Item> {
        for current in self.0.iter_mut().rev() {
            let (next, carry) = next_char(*current);
            *current = next;
            if !carry {
                break;
            }
        }
        Some(Password(self.0.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn password_display() {
        assert_eq!(
            Password::try_from("abcdefgh").unwrap().to_string(),
            "abcdefgh"
        );
    }

    #[test]
    fn password_ensures_len() {
        assert!(Password::try_from("abc").is_err());
        assert!(Password::try_from("abcdefghi").is_err());
    }

    #[test]
    fn password_ensures_lowercase_letters() {
        assert!(Password::try_from("ABCDEFGH").is_err());
    }

    #[test]
    fn password_iter_after() {
        assert_eq!(
            PasswordIter::after(Password::try_from("aaaaaaaa").unwrap()).next(),
            Some(Password::try_from("aaaaaaab").unwrap())
        );
        assert_eq!(
            PasswordIter::after(Password::try_from("aaaaazzz").unwrap()).next(),
            Some(Password::try_from("aaaabaaa").unwrap())
        );
        assert_eq!(
            PasswordIter::after(Password::try_from("zzzzzzzz").unwrap()).next(),
            Some(Password::try_from("aaaaaaaa").unwrap())
        );
    }
}
