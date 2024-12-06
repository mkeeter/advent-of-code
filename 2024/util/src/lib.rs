struct GetIntegers<'a, T> {
    bytes: &'a [u8],
    index: usize,
    _tag: std::marker::PhantomData<fn() -> T>,
}

impl<T> Iterator for GetIntegers<'_, T>
where
    T: From<u8> + std::ops::Mul<T, Output = T> + std::ops::Add<T, Output = T>,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.bytes.len() {
            let mut v = T::from(0);
            while self.index < self.bytes.len()
                && self.bytes[self.index].is_ascii_digit()
            {
                v = v * T::from(10) + T::from(self.bytes[self.index] - b'0');
                self.index += 1;
            }
            while self.index < self.bytes.len()
                && !self.bytes[self.index].is_ascii_digit()
            {
                self.index += 1;
            }
            Some(v)
        } else {
            None
        }
    }
}

/// Returns a list of integers that appear in the string
///
/// This function does not check for overflow; it's recommended to compile with
/// `overflow-checks = true` (even in release mode) for additional safety.
pub fn get_integers<T>(s: &str) -> impl Iterator<Item = T> + '_
where
    T: 'static
        + From<u8>
        + std::ops::Mul<T, Output = T>
        + std::ops::Add<T, Output = T>,
{
    GetIntegers::<'_, T> {
        bytes: s.as_bytes(),
        index: 0,
        _tag: std::marker::PhantomData,
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct Grid<'a> {
    bytes: &'a [u8],
    width: usize,
    height: usize,
    empty: u8,
}

impl std::ops::Index<(i64, i64)> for Grid<'_> {
    type Output = u8;
    #[inline]
    fn index(&self, index: (i64, i64)) -> &Self::Output {
        let (x, y) = index;
        self.get(x, y).unwrap_or(&self.empty)
    }
}

impl<'a> Grid<'a> {
    pub fn new(s: &'a str) -> Self {
        assert!(s.is_ascii());
        let mut width = None;
        let mut height = 0;
        for row in s.lines() {
            let w = row.len();
            let prev = *width.get_or_insert(w);
            assert_eq!(prev, w);
            height += 1;
        }
        Self {
            bytes: s.as_bytes(),
            width: width.unwrap_or(0),
            height,
            empty: b'.',
        }
    }

    #[inline]
    pub fn get(&self, x: i64, y: i64) -> Option<&u8> {
        if x < 0
            || y < 0
            || x as usize >= self.width
            || y as usize >= self.height
        {
            None
        } else {
            Some(&self.bytes[(x as usize) + (y as usize) * (self.width + 1)])
        }
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }
}
