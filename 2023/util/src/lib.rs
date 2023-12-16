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

    /// Looks up a value in the grid
    #[inline]
    pub fn get(&self, pos: &(i64, i64)) -> Option<&char> {
        let i = self.index(*pos)?;
        Some(&self.data[i]).filter(|c| **c != '.')
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
