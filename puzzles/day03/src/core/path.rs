use super::{Direction, Location};

pub fn iter_path<'a, I>(directions: I) -> impl Iterator<Item = Location> + 'a
where
    I: IntoIterator<Item = &'a Direction>,
    I::IntoIter: 'a,
{
    let mut location = Location::default();

    std::iter::once(location).chain(directions.into_iter().map(move |direction| {
        location += direction.offset();
        location
    }))
}

#[cfg(test)]
mod tests {
    use crate::core::{Direction, Location};

    use super::*;

    use Direction::*;

    #[test]
    fn test_iter_path() {
        macro_rules! test {
            ($input:expr, $expected:expr) => {
                assert_eq!(
                    iter_path(&$input).collect::<Vec<_>>(),
                    $expected
                        .into_iter()
                        .map(Location::from)
                        .collect::<Vec<_>>()
                );
            };
        }

        test!([], [[0, 0]]);
        test!([East], [[0, 0], [1, 0]]);
        test!(
            [North, East, South, West],
            [[0, 0], [0, 1], [1, 1], [1, 0], [0, 0]]
        );
        test!(
            [South, West, North, East],
            [[0, 0], [0, -1], [-1, -1], [-1, 0], [0, 0]]
        );
    }
}
