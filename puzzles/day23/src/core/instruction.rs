use std::convert::TryFrom;

use anyhow::{bail, ensure, Context};

use super::{Offset, RegisterKey, NUM_REGISTERS};

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Half(RegisterKey),
    Triple(RegisterKey),
    Increment(RegisterKey),
    Jump(Offset),
    JumpIfEven(RegisterKey, Offset),
    JumpIfOne(RegisterKey, Offset),
}

impl TryFrom<&str> for Instruction {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let (left, right) = s
            .split_once(|c: char| c.is_whitespace())
            .context("instruction name and data should be delimited with whitespace")?;

        let name = left.trim();
        let mut args = right.trim().split(',').map(|s| s.trim());

        fn take_register_key<'a>(
            args: &mut impl Iterator<Item = &'a str>,
        ) -> anyhow::Result<RegisterKey> {
            let s = args.next().context("missing register key")?;
            let mut chars = s.chars();

            let c = chars
                .next()
                .context("register key should be at least one character")?;

            ensure!(
                chars.next().is_none(),
                "register key should be a single character"
            );

            ensure!(
                c.is_ascii_alphabetic() && c.is_ascii_lowercase(),
                "invalid register key: {:?}",
                c
            );

            let key = (c as usize) - ('a' as usize);

            ensure!(
                key < NUM_REGISTERS,
                "register key exceeds register size: {:?}",
                c
            );

            Ok(key)
        }

        fn take_offset<'a>(args: &mut impl Iterator<Item = &'a str>) -> anyhow::Result<Offset> {
            let s = args.next().context("missing offset")?;

            let (sign, s) = s.split_at(1);

            let sign = match sign {
                "+" => 1,
                "-" => -1,
                _ => {
                    bail!("offset sign should be + or -, but it was {:?}", sign);
                }
            };

            let magnitude: isize = s.parse()?;

            Ok(sign * magnitude)
        }

        let instruction = match name {
            "hlf" => Self::Half(take_register_key(&mut args)?),
            "tpl" => Self::Triple(take_register_key(&mut args)?),
            "inc" => Self::Increment(take_register_key(&mut args)?),
            "jmp" => Self::Jump(take_offset(&mut args)?),
            "jie" => Self::JumpIfEven(take_register_key(&mut args)?, take_offset(&mut args)?),
            "jio" => Self::JumpIfOne(take_register_key(&mut args)?, take_offset(&mut args)?),
            _ => {
                bail!("invalid instruction: {:?}", name);
            }
        };

        ensure!(args.next().is_none(), "unexpected arguments");

        Ok(instruction)
    }
}
