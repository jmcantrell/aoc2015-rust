use std::iter::once;

use itertools::Itertools;

// https://en.wikipedia.org/wiki/Stars_and_bars_(combinatorics)
// https://stackoverflow.com/a/28969798 (adapted from python implementation)
pub fn iter_partitions(n: usize, k: usize) -> impl Iterator<Item = Vec<usize>> {
    (1..=n + k - 1).combinations(k - 1).map(move |c| {
        once(0)
            .chain(c.clone())
            .zip(c.into_iter().chain(once(n + k)))
            .map(|(a, b)| b - a - 1)
            .collect::<Vec<_>>()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter_partitions() {
        macro_rules! test {
            ($n:expr, $k:expr, $expected:expr) => {
                assert_eq!(
                    iter_partitions($n, $k).collect::<Vec<_>>(),
                    $expected.into_iter().map(Vec::from).collect::<Vec<_>>()
                );
            };
        }

        test!(
            5,
            3,
            [
                [0, 0, 5],
                [0, 1, 4],
                [0, 2, 3],
                [0, 3, 2],
                [0, 4, 1],
                [0, 5, 0],
                [1, 0, 4],
                [1, 1, 3],
                [1, 2, 2],
                [1, 3, 1],
                [1, 4, 0],
                [2, 0, 3],
                [2, 1, 2],
                [2, 2, 1],
                [2, 3, 0],
                [3, 0, 2],
                [3, 1, 1],
                [3, 2, 0],
                [4, 0, 1],
                [4, 1, 0],
                [5, 0, 0]
            ]
        );

        dbg!(iter_partitions(100, 4).count());
    }
}
