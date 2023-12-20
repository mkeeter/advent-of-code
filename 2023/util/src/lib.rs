/// Dense, fixed-size character grid, which treats `'.'` as `None`
pub struct DenseGrid {
    data: Vec<char>,
    width: usize,
}

impl DenseGrid {
    /// Builds a new grid from input text
    ///
    /// # Panics
    /// If all lines are not the same length, or the input is empty
    pub fn new(s: &str) -> Self {
        let width = s.lines().next().expect("input must not be empty").len();
        let mut data = vec![];
        for line in s.lines() {
            assert_eq!(line.len(), width, "line length must be equal");
            data.extend(line.chars());
        }
        Self { data, width }
    }

    /// Checks whether this position is contained within the dense grid
    ///
    /// Positions outside the dense grid can still be checked with
    /// [`DenseGrid::get`], but will never contain a value.
    #[inline]
    pub fn contains(&self, pos: (i64, i64)) -> bool {
        self.index(pos).is_some()
    }

    /// Builds a new empty grid
    #[inline]
    pub fn empty(width: usize, height: usize) -> Self {
        let data = vec!['.'; width * height];
        Self { data, width }
    }

    /// Returns the height of the grid
    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the height of the grid
    #[inline]
    pub fn height(&self) -> usize {
        self.data.len() / self.width
    }

    /// Converts from a 2D position to an index in the dense grid
    ///
    /// Returns `None` if the position is outside the grid
    #[inline]
    pub fn index(&self, pos: (i64, i64)) -> Option<usize> {
        let x: usize = pos.0.try_into().ok()?;
        let y: usize = pos.1.try_into().ok()?;
        if x >= self.width {
            None
        } else {
            let i = y * self.width + x;
            if i >= self.data.len() {
                None
            } else {
                Some(i)
            }
        }
    }

    #[inline]
    fn deindex(&self, i: usize) -> (i64, i64) {
        let y = i / self.width;
        let x = i % self.width;
        (x as i64, y as i64)
    }

    /// Inserts a value into the grid
    ///
    /// # Panics
    /// If the position is outside the grid, or `c == '.'` (which is used
    /// internally as a marker for missing values)
    #[inline]
    pub fn insert(&mut self, pos: (i64, i64), c: char) {
        assert_ne!(c, '.');
        let i = self.index(pos).unwrap();
        self.data[i] = c
    }

    /// Looks up a value in the grid by index
    #[inline]
    pub fn get_by_index(&self, i: usize) -> Option<&char> {
        Some(&self.data[i]).filter(|c| **c != '.')
    }

    /// Looks up a value in the grid by (2D) position
    #[inline]
    pub fn get(&self, pos: &(i64, i64)) -> Option<&char> {
        let i = self.index(*pos)?;
        self.get_by_index(i)
    }

    /// Returns an iterator over grid values
    pub fn iter(&self) -> impl Iterator<Item = ((i64, i64), &char)> + '_ {
        self.data.iter().enumerate().filter_map(|(i, c)| {
            if *c == '.' {
                None
            } else {
                Some((self.deindex(i), c))
            }
        })
    }

    /// Filters grid values with the given predicate
    pub fn retain<F: FnMut(&(i64, i64), &char) -> bool>(&mut self, mut f: F) {
        let mut data = std::mem::take(&mut self.data);
        for (i, c) in data.iter_mut().enumerate() {
            let pos = self.deindex(i);
            if *c != '.' && !f(&pos, c) {
                *c = '.';
            }
        }
        self.data = data;
    }

    /// Calculates the bounds of the active grid
    ///
    /// The bounds may be smaller than the full dense grid, e.g. if all items at
    /// `y == 0` are absent.
    pub fn bounds(&self) -> Bounds {
        let mut xmin = i64::MAX;
        let mut xmax = i64::MIN;
        let mut ymin = i64::MAX;
        let mut ymax = i64::MIN;
        for (i, c) in self.data.iter().enumerate() {
            if *c != '.' {
                let (x, y) = self.deindex(i);
                xmin = xmin.min(x);
                xmax = xmax.max(x);
                ymin = ymin.min(y);
                ymax = ymax.max(y);
            }
        }
        Bounds {
            xmin,
            ymin,
            xmax,
            ymax,
        }
    }
}

/// 2D bounds
pub struct Bounds {
    pub xmin: i64,
    pub ymin: i64,
    pub xmax: i64,
    pub ymax: i64,
}

/// 2D directions
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    /// Move in the given direction
    pub fn next(&self, pos: (i64, i64)) -> (i64, i64) {
        let (x, y) = pos;
        match self {
            Direction::North => (x, y - 1),
            Direction::South => (x, y + 1),
            Direction::East => (x + 1, y),
            Direction::West => (x - 1, y),
        }
    }
    /// Return the direction as a bitmask
    pub fn bit(&self) -> u8 {
        match self {
            Direction::North => 0b0001,
            Direction::South => 0b0010,
            Direction::East => 0b0100,
            Direction::West => 0b1000,
        }
    }
}

/// Find the least common multiple of a set of values
pub fn lcm(mut nums: Vec<usize>) -> usize {
    while nums.len() > 1 {
        let pa = nums.pop().unwrap();
        let pb = nums.pop().unwrap();
        let mut a = pa;
        let mut b = pb;
        while a != b {
            if a < b {
                a += (b - a).div_ceil(pa) * pa;
            } else {
                b += (a - b).div_ceil(pb) * pb;
            }
        }
        nums.push(a);
    }
    nums[0]
}

/// Trivial hash-map for < 256 items
#[derive(Debug)]
pub struct FlatMap<T>([Option<T>; 256]);

impl<T> FlatMap<T> {
    pub fn new() -> Self {
        Self([(); 256].map(|_| None))
    }
    pub fn get_mut(&mut self, i: u8) -> Option<&mut T> {
        self.0[i as usize].as_mut()
    }
    pub fn get(&self, i: u8) -> Option<&T> {
        self.0[i as usize].as_ref()
    }
    pub fn iter(&self) -> impl Iterator<Item = (u8, &T)> {
        self.0
            .iter()
            .enumerate()
            .filter(|(_, t)| t.is_some())
            .map(|(i, t)| (i as u8, t.as_ref().unwrap()))
    }
    pub fn insert(&mut self, i: u8, t: T) {
        self.0[i as usize] = Some(t)
    }
    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.0.iter().filter_map(|v| v.as_ref())
    }
}

impl<T> Default for FlatMap<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> FromIterator<(u8, T)> for FlatMap<T> {
    fn from_iter<I: IntoIterator<Item = (u8, T)>>(iter: I) -> Self {
        let mut c = Self::new();
        for (i, t) in iter {
            c.insert(i, t);
        }
        c
    }
}

impl<T> std::ops::Index<u8> for FlatMap<T> {
    type Output = T;
    fn index(&self, i: u8) -> &Self::Output {
        self.get(i).unwrap()
    }
}

impl<T> std::ops::IndexMut<u8> for FlatMap<T> {
    fn index_mut(&mut self, i: u8) -> &mut Self::Output {
        self.get_mut(i).unwrap()
    }
}
