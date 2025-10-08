/// A 2D grid, where coordinates are expressed as a couple `(line, column)`.
///
/// The origin `(0,0)` is the top-left-most item.
/// The bottom-right-most item is at coordinates (height-1, width-1).
pub struct Grid<T = char> {
    pub lines: usize,
    pub columns: usize,
    pub items: Vec<T>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position(pub usize, pub usize);

impl Position {
    pub fn add(&self, delta: &Displacement) -> Option<Self> {
        if let Some(line) = self.0.checked_add_signed(delta.0) {
            if let Some(column) = self.1.checked_add_signed(delta.1) {
                return Some(Position(line, column));
            }
        }
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Displacement(pub isize, pub isize);

impl Displacement {
    pub fn is_identity(&self) -> bool {
        self.0 == 0 && self.1 == 0
    }

    pub const NORTH: Displacement = Displacement(-1, 0);
    pub const EAST: Displacement = Displacement(0, 1);
    pub const SOUTH: Displacement = Displacement(1, 0);
    pub const WEST: Displacement = Displacement(0, -1);

    pub fn turn_right(&self) -> Self {
        match self {
            &Self::NORTH => Self::EAST,
            &Self::EAST => Self::SOUTH,
            &Self::SOUTH => Self::WEST,
            &Self::WEST => Self::NORTH,
            _ => unimplemented!()
        }
    }
}

impl From<(i32, i32)> for Displacement {
    fn from(value: (i32, i32)) -> Self {
        Self(value.0 as isize, value.1 as isize)
    }
}

impl std::ops::Add for Displacement {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Displacement(self.0 + other.0, self.1 + other.1)
    }
}

impl<T> std::ops::Mul<T> for Displacement
where
    isize: std::ops::Mul<T>,
    T: std::ops::Mul<isize, Output = isize> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Displacement(rhs * self.0, rhs * self.1)
    }
}

/// all direction vectors `(delta-line, delta-column)`:
///
/// ```
///   o---> column
///   |
///   |
///   v
///  line
///
///
///  -1,-1 -1,0 -1,1
///       \  |  /
///        \ | /
/// 0,-1 <---o---> 0,1
///        / | \
///       /  |  \
///   1,-1  1,0  1,1
/// ```
pub const ALL_DIRECTIONS: [Displacement; 8] = [
    Displacement(0, 1),
    Displacement(1, 1),
    Displacement(1, 0),
    Displacement(1, -1),
    Displacement(0, -1),
    Displacement(-1, -1),
    Displacement(-1, 0),
    Displacement(-1, 1),
];

impl Grid<char> {
    /// Read a grid from the given string, lines are separated by ascii whitespace.
    pub fn new(input: &str) -> Self {
        let lines = input.split_ascii_whitespace().collect::<Vec<_>>();
        let height = lines.len();
        let width = lines.first().unwrap().len();
        let items = lines
            .iter()
            .flat_map(|&line| line.chars().collect::<Vec<_>>())
            .collect();
        Grid {
            lines: height,
            columns: width,
            items,
        }
    }
}

impl<T> Grid<T> {
    pub fn valid_position(&self, pos: &Position) -> bool {
        pos.0 < self.lines && pos.1 < self.columns
    }

    pub fn valid_coordinates(&self, line: usize, column: usize) -> bool {
        line < self.lines && column < self.columns
    }

    pub fn step(&self, origin: &Position, delta: &Displacement) -> Option<Position> {
        origin.add(delta).filter(|pos| self.valid_position(pos))
    }

    fn index_to_position(&self, index: usize) -> Position {
        assert!(index < self.items.len());
        Position(index / self.columns, index % self.columns)
    }

    /// Retrieve value at given line and column coordinates.
    pub fn at(&self, line: usize, column: usize) -> Option<&T> {
        if self.valid_coordinates(line, column) {
            let index = line * self.columns + column;
            self.items.get(index)
        } else {
            None
        }
    }

    /// Retrieve value at given position.
    pub fn get(&self, position: &Position) -> Option<&T> {
        if self.valid_position(position) {
            let index = position.0 * self.columns + position.1;
            self.items.get(index)
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, position: &Position) -> Option<&mut T> {
        if self.valid_position(position) {
            let index = position.0 * self.columns + position.1;
            self.items.get_mut(index)
        } else {
            None
        }
    }

    /// Search for an element, returning its index.
    pub fn position<P>(&self, predicate: P) -> Option<Position>
    where
        P: Fn(&T) -> bool,
    {
        self.items
            .iter()
            .enumerate()
            .find(|(_, val)| predicate(val))
            .map(|(i, _)| self.index_to_position(i))
    }

    /// Search for an element
    pub fn find<P>(&self, predicate: P) -> Option<&T>
    where
        P: Fn(&T) -> bool,
    {
        self.items.iter().find(|&x| predicate(x))
    }
}

impl<T> Grid<T>
where
    T: Default + Clone,
{
    /// Extract N items by applying the given step N-1 times starting from the given origin position.
    ///
    /// Return `None` if any generated coordinates is outside the grid's boundaries.
    pub fn step_extract<const N: usize>(
        &self,
        origin: &Position,
        step: &Displacement,
    ) -> Option<[T; N]> {
        let mut items: [T; N] = std::array::from_fn(|_| T::default());

        for i in 0..N {
            let displacement = *step * (i as isize);
            if let Some(pos) = origin.add(&displacement) {
                if let Some(item) = self.get(&pos).cloned() {
                    items[i] = item;
                } else {
                    return None;
                }
            } else {
                return None;
            }
        }
        Some(items)
    }

    /// Extract N items by applying the given deltas to the given origin.
    ///
    /// Return `None` if any generated coordinates is outside the grid's boundaries.
    pub fn deltas_extract<const N: usize>(
        &self,
        origin: &Position,
        deltas: [Displacement; N],
    ) -> Option<[T; N]> {
        let mut items: [T; N] = std::array::from_fn(|_| T::default());
        for (i, d) in deltas.iter().enumerate() {
            if let Some(pos) = origin.add(d) {
                if let Some(item) = self.get(&pos).cloned() {
                    items[i] = item;
                } else {
                    return None;
                }
            } else {
                return None;
            }
        }

        Some(items)
    }

    pub fn new_from<B, F>(&self, f: F) -> Grid<B>
    where
        F: Fn(&T) -> B,
    {
        Grid {
            lines: self.lines,
            columns: self.columns,
            items: self.items.iter().map(f).collect(),
        }
    }
}
