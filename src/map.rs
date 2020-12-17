use std::{collections::HashSet, fmt::Debug, iter::FromIterator, ops::Add};

#[derive(Debug, Clone)]
pub struct Map<const N: usize> {
    inner: HashSet<Coord<N>>,
}

impl<const N: usize> Map<N> {
    pub fn new() -> Self {
        Self {
            inner: HashSet::new(),
        }
    }

    pub fn get_cube(&self, coord: Coord<N>) -> Cube {
        if self.inner.contains(&coord) {
            Cube::Active
        } else {
            Cube::Inactive
        }
    }

    pub fn set_cube(&mut self, coord: Coord<N>, cube: Cube) {
        match cube {
            Cube::Active => self.inner.insert(coord),
            Cube::Inactive => self.inner.remove(&coord),
        };
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }

    pub fn active_cubes(&self) -> ActiveCubes<N> {
        ActiveCubes {
            inner: self.inner.iter(),
        }
    }

    pub fn neighbors_of(&self, coord: Coord<N>) -> NeighborsOf<N> {
        NeighborsOf {
            current: [-1; N],
            coord,
            done: false,
        }
    }
}

impl<const N: usize> FromIterator<(Coord<N>, Cube)> for Map<N> {
    fn from_iter<T: IntoIterator<Item = (Coord<N>, Cube)>>(iter: T) -> Self {
        iter.into_iter().fold(Map::new(), |mut map, (coord, cube)| {
            map.set_cube(coord, cube);
            map
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord<const N: usize> {
    pub dimensions: [i32; N],
}

impl<const N: usize> Add<Coord<N>> for Coord<N> {
    type Output = Coord<N>;

    fn add(self, rhs: Coord<N>) -> Self::Output {
        let mut new_dims = [0; N];
        for index in 0..N {
            new_dims[index] = self.dimensions[index] + rhs.dimensions[index];
        }
        Coord {
            dimensions: new_dims,
        }
    }
}

impl<const N: usize> Debug for Coord<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;
        for index in 0..N - 1 {
            write!(f, "{}, ", self.dimensions[index])?;
        }
        write!(f, "{})", self.dimensions[N - 1])?;

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cube {
    Active,
    Inactive,
}

pub struct NeighborsOf<const N: usize> {
    current: [i32; N],
    coord: Coord<N>,
    done: bool,
}

impl<const N: usize> Iterator for NeighborsOf<N> {
    type Item = Coord<N>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let out = self.current;
        let mut index = N - 1;
        loop {
            if self.current[index] + 1 > 1 {
                self.current[index] = -1;
                index = match index.checked_sub(1) {
                    Some(index) => index,
                    None => {
                        self.done = true;
                        return Some(self.coord + Coord { dimensions: [1; N] });
                    }
                };
            } else {
                self.current[index] += 1;
                if self.current == [0; N] {
                    continue;
                };
                break;
            }
        }
        Some(self.coord + Coord { dimensions: out })
    }
}

pub struct ActiveCubes<'a, const N: usize> {
    inner: std::collections::hash_set::Iter<'a, Coord<N>>,
}

impl<'a, const N: usize> Iterator for ActiveCubes<'a, N> {
    type Item = Coord<N>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().copied()
    }
}
