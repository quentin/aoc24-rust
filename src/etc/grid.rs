//! 2D grid stuff.
#![allow(dead_code)]
use std::ops::Add;

/// A 2D grid, where coordinates are expressed as a couple `(line, column)`.
///
/// The origin `(0,0)` is the top-left-most item.
/// The bottom-right-most item is at coordinates (height-1, width-1).
#[derive(Clone)]
pub struct Grid<T = char> {
    pub lines: usize,
    pub columns: usize,
    pub items: Vec<T>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point(pub i64, pub i64);

impl Point {
    pub fn is_identity(&self) -> bool {
        self.0 == 0 && self.1 == 0
    }

    pub const NORTH: Point = Point(-1, 0);
    pub const EAST: Point = Point(0, 1);
    pub const SOUTH: Point = Point(1, 0);
    pub const WEST: Point = Point(0, -1);
    pub const NORTH_EAST: Point = Point(-1, 1);
    pub const NORTH_WEST: Point = Point(-1, -1);
    pub const SOUTH_EAST: Point = Point(1, 1);
    pub const SOUTH_WEST: Point = Point(1, -1);

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

impl<T: std::fmt::Debug> std::fmt::Debug for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.items.iter().enumerate().for_each(|(idx, val)| {
            val.fmt(f).unwrap();
            if (idx % self.columns) == (self.columns - 1) {
                f.write_str("\n").unwrap();
            }
        });
        Ok(())
    }
}

impl From<(i32, i32)> for Point {
    fn from(value: (i32, i32)) -> Self {
        Self(value.0.into(), value.1.into())
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
    i64: std::ops::Mul<T>,
    T: std::ops::Mul<i64, Output = i64> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Point(rhs * self.0, rhs * self.1)
    }
}

impl std::ops::Rem for Point {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Point(self.0 % rhs.0, self.1 % rhs.1)
    }
}

/// all direction vectors `(delta-line, delta-column)`:
///
/// ```test
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
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.items.iter()
    }

    pub fn valid_position(&self, pos: &Point) -> bool {
        pos.0 >= 0 && (pos.0 as usize) < self.lines && pos.1 >= 0 && (pos.1 as usize) < self.columns
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

    /// Unchecked conversion from cell index to point.
    pub fn unchecked_position(&self, index: usize) -> Point {
        Point((index / self.columns) as i64, (index % self.columns) as i64)
    }

    pub fn checked_position(&self, index: usize) -> Option<Point> {
        if self.valid_index(index) {
            Some(self.unchecked_position(index))
        } else {
            None
        }
    }

    pub fn strict_position(&self, index: usize) -> Point {
        if self.valid_index(index) {
            self.unchecked_position(index)
        } else {
            panic!("invalid index")
        }
    }

    pub fn unchecked_index(&self, pos: &Point) -> usize {
        self.columns * (pos.0 as usize) + (pos.1 as usize)
    }

    pub fn checked_index(&self, pos: &Point) -> Option<usize> {
        if self.valid_position(pos) {
            Some(self.unchecked_index(pos))
        } else {
            None
        }
    }

    pub fn strict_index(&self, pos: &Point) -> usize {
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

    /// Retrieve value at given point.
    pub fn get(&self, pos: &Point) -> Option<&T> {
        self.checked_index(pos)
            .map(|index| self.items.get(index).unwrap())
    }

    pub fn strict_get(&self, pos: &Point) -> &T {
        self.items.get(self.strict_index(pos)).unwrap()
    }

    pub fn unchecked_get(&self, pos: &Point) -> &T {
        self.items.get(self.unchecked_index(pos)).unwrap()
    }

    pub fn get_mut(&mut self, pos: &Point) -> Option<&mut T> {
        self.checked_index(pos)
            .map(|index| self.items.get_mut(index).unwrap())
    }

    pub fn set_at(&mut self, index: usize, val: T) {
        self.items[index] = val;
    }

    /// Search for an element, returning its index.
    pub fn position<P>(&self, predicate: P) -> Option<Point>
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
        F: FnMut(Point, &T),
    {
        self.items
            .iter()
            .enumerate()
            .for_each(|(index, item)| f(self.unchecked_position(index), item));
    }

    pub fn for_each_with_index<F>(&self, mut f: F)
    where
        F: FnMut(usize, &T),
    {
        self.items
            .iter()
            .enumerate()
            .for_each(|(index, item)| f(index, item));
    }

    pub fn step(&self, origin: &Point, delta: &Point) -> Option<Point> {
        let point = origin.add(*delta);
        self.valid_position(&point).then_some(point)
    }

    pub fn for_each_neighbour<F>(&self, origin: &Point, mut f: F)
    where
        F: FnMut(Point, &T),
    {
        for delta in &[Point::NORTH, Point::EAST, Point::SOUTH, Point::WEST] {
            if let Some(pos) = self.step(origin, delta) {
                f(pos, self.unchecked_get(&pos));
            }
        }
    }
}

impl<T> Grid<T>
where
    T: Default + Clone,
{
    pub fn default(lines: usize, columns: usize) -> Self {
        Grid {
            lines: lines,
            columns: columns,
            items: vec![T::default(); lines * columns],
        }
    }

    /// Extract N items by applying the given step N-1 times starting from the given origin position.
    ///
    /// Return `None` if any generated coordinates is outside the grid's boundaries.
    pub fn step_extract<const N: usize>(&self, origin: &Point, step: &Point) -> Option<[T; N]> {
        let mut items: [T; N] = std::array::from_fn(|_| T::default());

        for i in 0..N {
            let displacement = *step * (i as i64);
            let point = origin.add(displacement);
            if self.valid_position(&point) {
                if let Some(item) = self.get(&point).cloned() {
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
        origin: &Point,
        deltas: [Point; N],
    ) -> Option<[T; N]> {
        let mut items: [T; N] = std::array::from_fn(|_| T::default());
        for (i, d) in deltas.iter().enumerate() {
            let pos = origin.add(*d);
            if self.valid_position(&pos) {
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

    pub fn update_each<F>(&mut self, f: F)
    where
        F: Fn(&mut T),
    {
        self.items.iter_mut().for_each(f);
    }
}

impl<T> Grid<T>
where
    T: Copy,
{
    pub fn update(&mut self, pos: &Point, v: T) -> Option<T> {
        self.get_mut(pos).map(|cell| {
            let old = *cell;
            *cell = v;
            old
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{Grid, Point};
    #[test]
    fn rotate_90_clockwise() {
        assert_eq!(Point::NORTH.rotate_90_clockwise(), Point::EAST);
        assert_eq!(Point::EAST.rotate_90_clockwise(), Point::SOUTH);
        assert_eq!(Point::SOUTH.rotate_90_clockwise(), Point::WEST);
        assert_eq!(Point::WEST.rotate_90_clockwise(), Point::NORTH);
    }

    #[test]
    fn rotate_90_counterclockwise() {
        assert_eq!(Point::NORTH.rotate_90_counterclockwise(), Point::WEST);
        assert_eq!(Point::EAST.rotate_90_counterclockwise(), Point::NORTH);
        assert_eq!(Point::SOUTH.rotate_90_counterclockwise(), Point::EAST);
        assert_eq!(Point::WEST.rotate_90_counterclockwise(), Point::SOUTH);
    }

    #[test]
    fn rotate_180() {
        assert_eq!(Point::NORTH.rotate_180(), Point::SOUTH);
        assert_eq!(Point::EAST.rotate_180(), Point::WEST);
        assert_eq!(Point::SOUTH.rotate_180(), Point::NORTH);
        assert_eq!(Point::WEST.rotate_180(), Point::EAST);
    }

    #[test]
    fn is_identity() {
        assert!(Point(0, 0).is_identity());
        assert!(!Point::NORTH.is_identity());
    }

    #[test]
    fn valid_index() {
        let g = Grid::new("1234\n5678\n");
        assert!(g.valid_index(0));
        assert!(g.valid_index(1));
        assert!(g.valid_index(7));
        assert!(g.valid_index(g.size() - 1));
        assert!(!g.valid_index(g.size()));
    }

    #[test]
    fn checked_position() {
        let g = Grid::new("1234\n5678\n");
        assert_eq!(Some(Point(0, 0)), g.checked_position(0));
        assert_eq!(Some(Point(1, 1)), g.checked_position(5));
        assert_eq!(None, g.checked_position(g.size()));
        assert_eq!(None, g.checked_position(100));
    }

    #[test]
    fn strict_position() {
        let g = Grid::new("1234\n5678\n");
        assert_eq!(Point(0, 0), g.strict_position(0));
        assert_eq!(Point(1, 1), g.strict_position(5));
    }

    #[test]
    #[should_panic]
    fn strict_position_panics() {
        let g = Grid::new("1234\n5678\n");
        g.strict_position(g.size());
    }

    #[test]
    #[should_panic]
    fn strict_index_panics() {
        let g = Grid::new("1234\n5678\n");
        g.strict_index(&Point(2, 0));
    }

    #[test]
    #[should_panic]
    fn strict_get_panics() {
        let g = Grid::new("1234\n5678\n");
        g.strict_get(&Point(2, 0));
    }

    #[test]
    fn find() {
        let g = Grid::new("1234\n5678\n");
        assert_eq!(None, g.find(|v| *v == '0'));
        assert_eq!(Some(&'8'), g.find(|v| *v == '8'));
    }

    #[test]
    fn position() {
        let g = Grid::new("1234\n5678\n");
        assert_eq!(None, g.position(|v| *v == '0'));
        assert_eq!(Some(Point(1, 3)), g.position(|v| *v == '8'));
    }
}
