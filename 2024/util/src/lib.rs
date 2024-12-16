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
            self.skip_non_ascii();
            Some(v)
        } else {
            None
        }
    }
}

impl<T> GetIntegers<'_, T> {
    fn skip_non_ascii(&mut self) {
        while self.index < self.bytes.len()
            && !self.bytes[self.index].is_ascii_digit()
        {
            self.index += 1;
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
    let mut out = GetIntegers::<'_, T> {
        bytes: s.as_bytes(),
        index: 0,
        _tag: std::marker::PhantomData,
    };
    out.skip_non_ascii();
    out
}

////////////////////////////////////////////////////////////////////////////////

/// Dense immutable grid based on a borrowed string
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
    pub fn width(&self) -> i64 {
        self.width as i64
    }

    #[inline]
    pub fn height(&self) -> i64 {
        self.height as i64
    }
}

////////////////////////////////////////////////////////////////////////////////

/// Dense fixed-length bitset
pub struct BitSet(Vec<u64>);
impl BitSet {
    #[inline]
    pub fn new(size: usize) -> Self {
        Self(vec![0u64; size.div_ceil(64)])
    }
    #[inline]
    pub fn get(&self, i: usize) -> bool {
        (self.0[i / 64] & (1 << (i % 64))) != 0
    }
    #[inline]
    pub fn set(&mut self, i: usize) {
        self.0[i / 64] |= 1 << (i % 64)
    }
    /// Inserts `true` at the given position
    ///
    /// Returns whether the value was newly inserted
    #[inline]
    pub fn insert(&mut self, i: usize) -> bool {
        let prev = self.get(i);
        self.set(i);
        !prev
    }
    #[inline]
    pub fn len(&self) -> usize {
        self.0.iter().map(|b| b.count_ones() as usize).sum()
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.iter().all(|b| *b == 0)
    }
    #[inline]
    pub fn clear(&mut self) {
        self.0.fill(0)
    }
}

/// Dense fixed-length bitset for a grid
pub struct GridSet(TupleSet<(i64, i64)>);
impl GridSet {
    #[inline]
    pub fn new(g: &Grid) -> Self {
        Self(TupleSet::new((g.width(), g.height())))
    }
    #[inline]
    pub fn from_width_and_height(width: i64, height: i64) -> Self {
        Self(TupleSet::new((width, height)))
    }
    /// Inserts `true` at the given position
    ///
    /// Returns whether the value was newly inserted
    #[inline]
    pub fn insert(&mut self, x: i64, y: i64) -> bool {
        self.0.insert((x, y))
    }
    #[inline]
    pub fn contains(&self, x: i64, y: i64) -> bool {
        self.0.contains((x, y))
    }
    #[inline]
    pub fn clear(&mut self) {
        self.0.clear()
    }
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

pub struct TupleSet<T> {
    data: BitSet,
    sizes: T,
}

impl<T: SizedTuple> TupleSet<T> {
    #[inline]
    pub fn new(sizes: T) -> Self {
        let mut total_size = 1;
        for i in 0..T::LEN {
            total_size *= sizes.get(i);
        }
        Self {
            data: BitSet::new(total_size),
            sizes,
        }
    }

    #[inline]
    fn get_index(&self, k: T) -> usize {
        let mut index = 0;
        for i in 0..T::LEN {
            assert!(k.get(i) < self.sizes.get(i));
            index = index * self.sizes.get(i) + k.get(i);
        }
        index
    }

    /// Inserts `true` at the given position
    ///
    /// Returns whether the value was newly inserted
    #[inline]
    pub fn insert(&mut self, k: T) -> bool {
        self.data.insert(self.get_index(k))
    }

    #[inline]
    pub fn contains(&self, k: T) -> bool {
        self.data.get(self.get_index(k))
    }

    #[inline]
    pub fn clear(&mut self) {
        self.data.clear()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

#[allow(clippy::len_without_is_empty)]
pub trait SizedTuple {
    const LEN: usize;
    fn get(&self, i: usize) -> usize;
}

impl<A, B> SizedTuple for (A, B)
where
    A: SizedTupleElement,
    B: SizedTupleElement,
{
    const LEN: usize = 2;

    #[inline]
    fn get(&self, i: usize) -> usize {
        match i {
            0 => self.0.get(),
            1 => self.1.get(),
            _ => panic!("invalid index {i}"),
        }
    }
}

impl<A, B, C> SizedTuple for (A, B, C)
where
    A: SizedTupleElement,
    B: SizedTupleElement,
    C: SizedTupleElement,
{
    const LEN: usize = 3;

    #[inline]
    fn get(&self, i: usize) -> usize {
        match i {
            0 => self.0.get(),
            1 => self.1.get(),
            2 => self.2.get(),
            _ => panic!("invalid index {i}"),
        }
    }
}

trait SizedTupleElement {
    fn get(&self) -> usize;
}

impl<T> SizedTupleElement for T
where
    T: TryInto<usize> + Copy,
    <T as TryInto<usize>>::Error: std::fmt::Debug,
{
    fn get(&self) -> usize {
        (*self).try_into().unwrap()
    }
}
