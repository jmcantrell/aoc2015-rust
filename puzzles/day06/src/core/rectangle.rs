use super::Location;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Rectangle {
    top_left: Location,
    bottom_right: Location,
}

impl Rectangle {
    pub fn new(top_left: Location, bottom_right: Location) -> Self {
        assert!(top_left.y <= bottom_right.y, "top row is after bottom row");
        assert!(
            top_left.x <= bottom_right.x,
            "left column is after right column"
        );

        Self {
            top_left,
            bottom_right,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = Location> + '_ {
        (self.top_left.y..=self.bottom_right.y).flat_map(|row| {
            (self.top_left.x..=self.bottom_right.x).map(move |column| [row, column].into())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter() {
        macro_rules! test {
            ($top_left:expr, $bottom_right:expr, $expected:expr) => {
                assert_eq!(
                    Rectangle::new($top_left.into(), $bottom_right.into())
                        .iter()
                        .collect::<Vec<_>>(),
                    Vec::from($expected.map(Location::from))
                );
            };
        }

        test!([0, 0], [0, 0], [[0, 0]]);
        test!([1, 1], [2, 2], [[1, 1], [1, 2], [2, 1], [2, 2]]);
    }
}
