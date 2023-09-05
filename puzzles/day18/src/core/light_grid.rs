use std::convert::TryFrom;

use anyhow::{anyhow, ensure, Context};

use super::{Cell, Direction, Grid, Location};

pub type Light = bool;
pub type LightCell<'a> = Cell<'a, Light>;

type Inner = Grid<Light>;

const DIRECTIONS: [Direction; 8] = [
    (-1, -1), // northwest
    (-1, 0),  // north
    (-1, 1),  // northeast
    (0, -1),  // west
    (0, 1),   // east
    (1, -1),  // southwest
    (1, 0),   // south
    (1, 1),   // southeast
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LightGrid(Inner);

fn neighbor((y, x): Location, (dy, dx): Direction) -> Option<Location> {
    y.checked_add_signed(dy).zip(x.checked_add_signed(dx))
}

fn iter_locations(height: usize, width: usize) -> impl Iterator<Item = Location> {
    (0..height).flat_map(move |row| (0..width).map(move |column| (row, column)))
}

impl LightGrid {
    fn neighbor(&self, location: Location, direction: Direction) -> Option<LightCell> {
        neighbor(location, direction)
            .and_then(|location| self.0.get(location))
            .map(move |value| (location, value))
    }

    fn neighbors(&self, location: Location) -> impl Iterator<Item = LightCell> + '_ {
        DIRECTIONS
            .into_iter()
            .filter_map(move |direction| self.neighbor(location, direction))
    }

    fn lit_neighbors(&self, location: Location) -> impl Iterator<Item = LightCell> {
        self.neighbors(location).filter(|&(_, &lit)| lit)
    }

    fn iter(&self) -> impl Iterator<Item = LightCell> {
        iter_locations(self.height(), self.height())
            .flat_map(|location| self.0.get(location).map(|value| (location, value)))
    }

    fn height(&self) -> usize {
        self.0.nrows()
    }

    fn width(&self) -> usize {
        self.0.ncols()
    }

    fn top(&self) -> usize {
        0
    }

    fn bottom(&self) -> usize {
        self.height() - 1
    }

    fn left(&self) -> usize {
        0
    }

    fn right(&self) -> usize {
        self.width() - 1
    }

    pub fn corners(&self) -> [Location; 4] {
        [
            (self.top(), self.left()),
            (self.top(), self.right()),
            (self.bottom(), self.left()),
            (self.bottom(), self.right()),
        ]
    }

    pub fn len_lit(&self) -> usize {
        self.0.iter().filter(|&&lit| lit).count()
    }

    pub fn animate<'a, F: 'a>(&'a self, evolve: F) -> impl Iterator<Item = Self> + '_
    where
        F: Fn(LightCell, usize) -> bool,
    {
        let mut prev = self.clone();
        let (height, width) = self.0.shape();

        std::iter::from_fn(move || {
            let lights = prev
                .iter()
                .map(|cell| evolve(cell, prev.lit_neighbors(cell.0).count()));

            let next = Self(Inner::from_row_iterator(height, width, lights));

            prev = next.clone();

            Some(next)
        })
    }
}

impl TryFrom<&str> for LightGrid {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        fn parse_light(c: char) -> anyhow::Result<bool> {
            match c {
                '#' => Ok(true),
                '.' => Ok(false),
                _ => Err(anyhow!("invalid light: {:?}", c)),
            }
        }

        let mut lines = s.lines().enumerate().peekable();
        let (_, first_line) = lines.peek().context("no lines")?;

        let width = first_line.len();

        let cells = lines
            .flat_map(|(i, s)| {
                s.chars().enumerate().map(move |(j, c)| {
                    parse_light(c)
                        .with_context(|| format!("line number {}, column number {}", i + 1, j + 1))
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        let height = cells.len() / width;

        ensure!(
            cells.len() == height * width,
            "expected cell count to be {}, but it was {}",
            height * width,
            cells.len()
        );

        let inner = Inner::from_row_iterator(height, width, cells.into_iter());

        Ok(Self(inner))
    }
}

impl std::fmt::Display for LightGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in self.0.row_iter() {
            for &lit in row.iter() {
                write!(f, "{}", if lit { '#' } else { '.' })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use nalgebra::dmatrix;

    use super::*;

    #[test]
    fn try_from_str() -> anyhow::Result<()> {
        assert_eq!(
            LightGrid::try_from(".#.#.#\n...##.\n#....#\n..#...\n#.#..#\n####..\n")?,
            LightGrid(dmatrix![
                    false, true, false, true, false, true;
                    false, false, false, true, true, false;
                    true, false, false, false, false, true;
                    false, false, true, false, false, false;
                    true, false, true, false, false, true;
                    true, true, true, true, false, false
            ])
        );
        Ok(())
    }

    #[test]
    fn animate() {
        let grid = LightGrid::try_from(".#.#.#\n...##.\n#....#\n..#...\n#.#..#\n####..\n").unwrap();
        let mut frames = grid.animate(|cell, lit_neighbors| match cell.1 {
            true => lit_neighbors == 2 || lit_neighbors == 3,
            false => lit_neighbors == 3,
        });

        macro_rules! test_next {
            ($expected:expr) => {
                assert_eq!(frames.next().unwrap().to_string(), $expected);
            };
        }

        test_next!("..##..\n..##.#\n...##.\n......\n#.....\n#.##..\n");
        test_next!("..###.\n......\n..###.\n......\n.#....\n.#....\n");
        test_next!("...#..\n......\n...#..\n..##..\n......\n......\n");
        test_next!("......\n......\n..##..\n..##..\n......\n......\n");
    }

    #[test]
    fn len_lit() {
        assert_eq!(
            LightGrid::try_from(".#.#.#\n...##.\n#....#\n..#...\n#.#..#\n####..\n")
                .unwrap()
                .len_lit(),
            15
        );
    }
}
