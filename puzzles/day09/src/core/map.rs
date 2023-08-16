use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use super::{Distance, Location, Route};

type Neighbors = HashMap<Location, HashSet<Location>>;
type Vertices = HashMap<(Location, Location), Distance>;
type Path<'a> = Vec<&'a Location>;

#[derive(Debug, Clone)]
pub struct Map {
    neighbors: Neighbors,
    vertices: Vertices,
}

impl Map {
    fn path_distance(&self, path: &Path) -> Option<Distance> {
        let mut distance = 0;
        let mut steps = path.iter();
        let mut prev = steps.next()?;

        for next in steps {
            distance += self.vertices.get(&(prev, next))?;
            prev = next;
        }

        Some(distance)
    }

    fn iter_paths(&self) -> impl Iterator<Item = (Path, Distance)> {
        self.neighbors
            .keys()
            .permutations(self.neighbors.len())
            .filter_map(|path| self.path_distance(&path).map(|distance| (path, distance)))
    }

    pub fn shortest_path(&self) -> Option<(Path, Distance)> {
        self.iter_paths().min_by_key(|&(_, distance)| distance)
    }

    pub fn longest_path(&self) -> Option<(Path, Distance)> {
        self.iter_paths().max_by_key(|&(_, distance)| distance)
    }
}

impl FromIterator<Route> for Map {
    fn from_iter<I: IntoIterator<Item = Route>>(iter: I) -> Self {
        let mut neighbors = Neighbors::new();
        let mut vertices = Vertices::new();

        for (location1, location2, distance) in iter {
            neighbors.entry(location1).or_default().insert(location2);
            neighbors.entry(location2).or_default().insert(location1);
            vertices.insert((location1, location2), distance);
            vertices.insert((location2, location1), distance);
        }

        Self {
            neighbors,
            vertices,
        }
    }
}
