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
    pub fn add(&self, delta: &Point) -> Option<Self> {
        if let Some(line) = self.0.checked_add_signed(delta.0) {
            if let Some(column) = self.1.checked_add_signed(delta.1) {
                return Some(Position(line, column));
            }
        }
        None
    }

    pub fn into_point(&self) -> Point {
        Point(self.0 as isize, self.1 as isize)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point(pub isize, pub isize);

impl Point {
    pub fn is_identity(&self) -> bool {
        self.0 == 0 && self.1 == 0
    }

    pub const NORTH: Point = Point(-1, 0);
    pub const EAST: Point = Point(0, 1);
    pub const SOUTH: Point = Point(1, 0);
    pub const WEST: Point = Point(0, -1);

    pub fn rotate_90_clockwise(&self) -> Self {
        Self(self.1, -self.0)
    }

    pub fn rotate_90_counterclockwise(&self) -> Self {
        Self(-self.1, self.0)
    }

    pub fn rotate_180(&self) -> Self {
        Self(-self.0, -self.1)
    }
}

impl From<(i32, i32)> for Point {
    fn from(value: (i32, i32)) -> Self {
        Self(value.0 as isize, value.1 as isize)
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Point(self.0 + other.0, self.1 + other.1)
    }
}

impl std::ops::Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Point(self.0 - other.0, self.1 - other.1)
    }
}

impl<T> std::ops::Mul<T> for Point
where
    isize: std::ops::Mul<T>,
    T: std::ops::Mul<isize, Output = isize> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Point(rhs * self.0, rhs * self.1)
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
pub const ALL_DIRECTIONS: [Point; 8] = [
    Point(0, 1),
    Point(1, 1),
    Point(1, 0),
    Point(1, -1),
    Point(0, -1),
    Point(-1, -1),
    Point(-1, 0),
    Point(-1, 1),
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

    pub fn valid_index(&self, index: usize) -> bool {
        index < self.items.len()
    }

    /// Return the number of cells.
    pub fn size(&self) -> usize {
        self.lines * self.columns
    }

    /// Unchecked conversion from cell index to position.
    pub fn unchecked_position(&self, index: usize) -> Position {
        Position(index / self.columns, index % self.columns)
    }

    pub fn checked_position(&self, index: usize) -> Option<Position> {
        if self.valid_index(index) {
            Some(self.unchecked_position(index))
        } else {
            None
        }
    }

    pub fn strict_position(&self, index: usize) -> Position {
        if self.valid_index(index) {
            self.unchecked_position(index)
        } else {
            panic!("invalid index")
        }
    }

    pub fn unchecked_index(&self, pos: &Position) -> usize {
        self.columns * pos.0 + pos.1
    }

    pub fn checked_index(&self, pos: &Position) -> Option<usize> {
        if self.valid_position(pos) {
            Some(self.unchecked_index(pos))
        } else {
            None
        }
    }

    pub fn strict_index(&self, pos: &Position) -> usize {
        if self.valid_position(pos) {
            self.unchecked_index(pos)
        } else {
            panic!("invalid position")
        }
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
    pub fn get(&self, pos: &Position) -> Option<&T> {
        self.checked_index(pos)
            .map(|index| self.items.get(index).unwrap())
    }

    pub fn strict_get(&self, pos: &Position) -> &T {
        self.items.get(self.strict_index(pos)).unwrap()
    }

    pub fn unchecked_get(&self, pos: &Position) -> &T {
        self.items.get(self.unchecked_index(pos)).unwrap()
    }

    pub fn get_mut(&mut self, pos: &Position) -> Option<&mut T> {
        self.checked_index(pos)
            .map(|index| self.items.get_mut(index).unwrap())
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
            .map(|(i, _)| self.unchecked_position(i))
    }

    /// Search for an element
    pub fn find<P>(&self, predicate: P) -> Option<&T>
    where
        P: Fn(&T) -> bool,
    {
        self.items.iter().find(|&x| predicate(x))
    }

    pub fn for_each_with_position<F>(&self, mut f: F)
    where
        F: FnMut(Position, &T),
    {
        self.items
            .iter()
            .enumerate()
            .for_each(|(index, item)| f(self.unchecked_position(index), item));
    }

    pub fn step(&self, origin: &Position, delta: &Point) -> Option<Position> {
        origin.add(delta).filter(|pos| self.valid_position(pos))
    }

    pub fn for_each_neighbour<F>(&self, origin: &Position, mut f: F)
    where
        F: FnMut(Position, &T),
    {
        for delta in &[Point::NORTH, Point::EAST, Point::SOUTH, Point::WEST] {
            if let Some(pos) = self.step(origin, delta){ 
                f(pos, self.unchecked_get(&pos));
            }
        }
    }
}

impl<T> Grid<T>
where
    T: Default + Clone,
{
    /// Extract N items by applying the given step N-1 times starting from the given origin position.
    ///
    /// Return `None` if any generated coordinates is outside the grid's boundaries.
    pub fn step_extract<const N: usize>(&self, origin: &Position, step: &Point) -> Option<[T; N]> {
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
        deltas: [Point; N],
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

#[cfg(test)]
mod tests {
    use super::Point;
    #[test]
    fn rotate_90_clockwise() {
        assert_eq!(Point::NORTH.rotate_90_clockwise(), Point::EAST);
        assert_eq!(Point::EAST.rotate_90_clockwise(), Point::SOUTH);
        assert_eq!(Point::SOUTH.rotate_90_clockwise(), Point::WEST);
        assert_eq!(Point::WEST.rotate_90_clockwise(), Point::NORTH);
    }

    #[test]
    fn rotate_180() {
        assert_eq!(Point::NORTH.rotate_180(), Point::SOUTH);
        assert_eq!(Point::EAST.rotate_180(), Point::WEST);
        assert_eq!(Point::SOUTH.rotate_180(), Point::NORTH);
        assert_eq!(Point::WEST.rotate_180(), Point::EAST);
    }
}
