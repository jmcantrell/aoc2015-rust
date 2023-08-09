use super::{Direction, Location};

pub fn iter_path<'a, I>(directions: I) -> impl Iterator<Item = Location> + 'a
where
    I: IntoIterator<Item = &'a Direction>,
    I::IntoIter: 'a,
{
    let mut floor = 0;

    directions.into_iter().map(move |direction| {
        floor += match direction {
            Direction::Up => 1,
            Direction::Down => -1,
        };
        floor
    })
}

#[cfg(test)]
pub mod tests {
    use crate::core::Direction::*;

    #[test]
    fn iter_path() {
        macro_rules! test {
            ($input:expr, $expected:expr) => {
                assert_eq!(
                    super::iter_path(&Vec::from($input)).collect::<Vec<_>>(),
                    Vec::from($expected)
                );
            };
        }

        test!([Up, Up, Down, Down], [1, 2, 1, 0]);
        test!([Up, Down, Up, Down], [1, 0, 1, 0]);
        test!([Up, Up, Up], [1, 2, 3]);
        test!([Up, Up, Down, Up, Up, Down, Up], [1, 2, 1, 2, 3, 2, 3]);
        test!([Down, Down, Up, Up, Up, Up, Up], [-1, -2, -1, 0, 1, 2, 3]);
        test!([Up, Down, Down], [1, 0, -1]);
        test!([Down, Down, Up], [-1, -2, -1]);
        test!([Down, Down, Down], [-1, -2, -3]);
        test!(
            [Down, Up, Down, Down, Up, Down, Down],
            [-1, 0, -1, -2, -1, -2, -3]
        );
    }
}
