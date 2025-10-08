/// A 2D grid, where coordinates are expressed as a couple `(line, column)`.
///
/// The origin `(0,0)` is the top-left-most item.
/// The bottom-right-most item is at coordinates (height-1, width-1).
pub struct Grid {
    pub lines: usize,
    pub columns: usize,
    pub items: Vec<char>,
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
pub const ALL_DIRECTIONS: [(isize, isize); 8] = [
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
];

impl Grid {
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

    pub fn get(&self, line: usize, column: usize) -> Option<&char> {
        let index = line * self.columns + column;
        self.items.get(index)
    }

    /// Extract N items by applying the given step N-1 times starting from the given origin position.
    ///
    /// Return `None` if any generated coordinates is outside the grid's boundaries.
    pub fn step_extract<const N: usize>(
        &self,
        origin: (usize, usize),
        step: (isize, isize),
    ) -> Option<[char; N]> {
        assert!(origin.0 < self.lines);
        assert!(origin.1 < self.columns);
        assert!(step.0 != 0 || step.1 != 0);

        let mut items: [char; N] = [char::default(); N];
        let (l0, c0) = (origin.0 as isize, origin.1 as isize);
        let (ls, cs) = step;

        for i in 0..N {
            let ld = i as isize * ls;
            let cd = i as isize * cs;
            let li = l0 + ld;
            let ci = c0 + cd;
            if li < 0 || li >= (self.columns as isize) {
                return None;
            }
            if ci < 0 || ci >= (self.lines as isize) {
                return None;
            }
            items[i] = self.get(li as usize, ci as usize).copied().unwrap();
        }
        Some(items)
    }

    /// Extract N items by applying the given deltas to the given origin.
    ///
    /// Return `None` if any generated coordinates is outside the grid's boundaries.
    pub fn deltas_extract<const N: usize>(
        &self,
        origin: (usize, usize),
        deltas: [(isize, isize); N],
    ) -> Option<[char; N]> {
        assert!(origin.0 < self.lines);
        assert!(origin.1 < self.columns);
        let mut items = [char::default(); N];
        let (l0, c0) = (origin.0 as isize, origin.1 as isize);
        for (i, (ld, cd)) in deltas.iter().enumerate() {
            let li = l0 + ld;
            let ci = c0 + cd;
            if li < 0 || li >= (self.columns as isize) {
                return None;
            }
            if ci < 0 || ci >= (self.lines as isize) {
                return None;
            }
            items[i] = self.get(li as usize, ci as usize).copied().unwrap();
        }

        Some(items)
    }
}
