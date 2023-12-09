use serde_json::Value;

use super::{Parsed1, Parsed2};

type Number = i64;
type Solution = Number;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

pub fn solve1(parsed: &Parsed1) -> anyhow::Result<Solution1> {
    fn recurse(value: &Value) -> Number {
        match value {
            Value::Number(number) => number.as_i64().unwrap(),
            Value::Array(array) => array.iter().map(recurse).sum(),
            Value::Object(object) => object.values().map(recurse).sum(),
            _ => 0,
        }
    }

    Ok(recurse(parsed))
}

pub fn solve2(parsed: &Parsed2) -> anyhow::Result<Solution2> {
    fn recurse(value: &Value) -> Number {
        match value {
            Value::Number(number) => number.as_i64().unwrap(),
            Value::Array(array) => array.iter().map(recurse).sum(),
            Value::Object(object) => {
                for value in object.values() {
                    if let Value::String(s) = value {
                        if s == "red" {
                            return 0;
                        }
                    }
                }
                object.values().map(recurse).sum()
            }
            _ => 0,
        }
    }

    Ok(recurse(parsed))
}

#[cfg(test)]
mod tests {
    use crate::answer::{parse1, parse2};

    use super::*;

    #[test]
    fn test_solve1() {
        macro_rules! test {
            ($input:expr, $expected:expr) => {
                assert_eq!(solve1(&parse1($input).unwrap()).unwrap(), $expected);
            };
        }

        test!("[1,2,3]", 6);
        test!("{\"a\":2,\"b\":4}", 6);
        test!("[[[3]]]", 3);
        test!("{\"a\":{\"b\":4},\"c\":-1}", 3);
        test!("{\"a\":[-1,1]}", 0);
        test!("[-1,{\"a\":1}]", 0);
        test!("[]", 0);
        test!("{}", 0);
    }

    #[test]
    fn test_solve2() {
        macro_rules! test {
            ($input:expr, $expected:expr) => {
                assert_eq!(solve2(&parse2($input).unwrap()).unwrap(), $expected);
            };
        }

        test!("[1,2,3]", 6);
        test!("[1,{\"c\":\"red\",\"b\":2},3]", 4);
        test!("{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}", 0);
        test!("[1,\"red\",5]", 6);
    }
}
