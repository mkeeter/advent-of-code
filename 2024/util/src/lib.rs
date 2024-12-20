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
#[derive(Copy, Clone)]
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
pub struct BitSet {
    data: Vec<u64>,
    size: usize,
}
impl BitSet {
    #[inline]
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0u64; size.div_ceil(64)],
            size,
        }
    }
    #[inline]
    pub fn get(&self, i: usize) -> bool {
        assert!(i < self.size);
        (self.data[i / 64] & (1 << (i % 64))) != 0
    }
    #[inline]
    pub fn set(&mut self, i: usize) {
        assert!(i < self.size);
        self.data[i / 64] |= 1 << (i % 64)
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
        self.data.iter().map(|b| b.count_ones() as usize).sum()
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.iter().all(|b| *b == 0)
    }
    #[inline]
    pub fn clear(&mut self) {
        self.data.fill(0)
    }

    #[inline]
    pub fn iter(&self) -> BitSetIter<'_> {
        BitSetIter {
            data: self,
            index: 0,
        }
    }
}

pub struct BitSetIter<'a> {
    data: &'a BitSet,
    index: usize,
}

impl Iterator for BitSetIter<'_> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.data.size {
            if self.data.get(self.index) {
                let prev = self.index;
                self.index += 1;
                return Some(prev);
            }
            self.index += 1;
        }
        None
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

impl<T: SizedTuple + Copy> TupleSet<T> {
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

    #[inline]
    pub fn iter(&self) -> TupleSetIter<'_, T> {
        TupleSetIter {
            iter: self.data.iter(),
            sizes: self.sizes,
        }
    }
}

pub struct TupleSetIter<'a, T> {
    iter: BitSetIter<'a>,
    sizes: T,
}

impl<T: SizedTuple> Iterator for TupleSetIter<'_, T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|mut v| {
            let mut out = T::zero();
            for i in (0..T::LEN).rev() {
                out.set(i, v % self.sizes.get(i));
                v /= self.sizes.get(i);
            }
            out
        })
    }
}

#[allow(clippy::len_without_is_empty)]
pub trait SizedTuple {
    const LEN: usize;
    fn zero() -> Self;
    fn get(&self, i: usize) -> usize;
    fn set(&mut self, i: usize, v: usize);
}

impl<A, B> SizedTuple for (A, B)
where
    A: SizedTupleElement,
    B: SizedTupleElement,
{
    const LEN: usize = 2;

    #[inline]
    fn zero() -> Self {
        (A::zero(), B::zero())
    }

    #[inline]
    fn get(&self, i: usize) -> usize {
        match i {
            0 => self.0.get(),
            1 => self.1.get(),
            _ => panic!("invalid index {i}"),
        }
    }

    #[inline]
    fn set(&mut self, i: usize, v: usize) {
        match i {
            0 => self.0 = A::set(v),
            1 => self.1 = B::set(v),
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
    fn zero() -> Self {
        (A::zero(), B::zero(), C::zero())
    }

    #[inline]
    fn get(&self, i: usize) -> usize {
        match i {
            0 => self.0.get(),
            1 => self.1.get(),
            2 => self.2.get(),
            _ => panic!("invalid index {i}"),
        }
    }

    #[inline]
    fn set(&mut self, i: usize, v: usize) {
        match i {
            0 => self.0 = A::set(v),
            1 => self.1 = B::set(v),
            2 => self.2 = C::set(v),
            _ => panic!("invalid index {i}"),
        }
    }
}

trait SizedTupleElement {
    fn zero() -> Self;
    fn set(v: usize) -> Self;
    fn get(&self) -> usize;
}

impl<T> SizedTupleElement for T
where
    T: TryInto<usize> + Copy + TryFrom<usize>,
    <T as TryInto<usize>>::Error: std::fmt::Debug,
    <T as TryFrom<usize>>::Error: std::fmt::Debug,
{
    fn zero() -> Self {
        T::try_from(0usize).unwrap()
    }
    fn set(v: usize) -> Self {
        Self::try_from(v).unwrap()
    }
    fn get(&self) -> usize {
        (*self).try_into().unwrap()
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Dir {
    N,
    E,
    S,
    W,
}

impl TryFrom<char> for Dir {
    type Error = char;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        Ok(match c {
            'v' => Self::S,
            '^' => Self::N,
            '>' => Self::E,
            '<' => Self::W,
            _ => return Err(c),
        })
    }
}

impl Dir {
    pub fn iter() -> impl Iterator<Item = Self> {
        [Dir::N, Dir::E, Dir::S, Dir::W].into_iter()
    }
    pub fn left(&self) -> Self {
        match self {
            Dir::N => Dir::E,
            Dir::E => Dir::S,
            Dir::S => Dir::W,
            Dir::W => Dir::N,
        }
    }
    pub fn right(&self) -> Self {
        match self {
            Dir::E => Dir::N,
            Dir::S => Dir::E,
            Dir::W => Dir::S,
            Dir::N => Dir::W,
        }
    }
    pub fn x(&self) -> i64 {
        match self {
            Dir::E => 1,
            Dir::W => -1,
            Dir::S | Dir::N => 0,
        }
    }
    pub fn y(&self) -> i64 {
        match self {
            Dir::E | Dir::W => 0,
            Dir::S => 1,
            Dir::N => -1,
        }
    }
    pub fn index(&self) -> usize {
        match self {
            Dir::N => 0,
            Dir::E => 1,
            Dir::S => 2,
            Dir::W => 3,
        }
    }
}

impl std::fmt::Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Dir::N => 'N',
                Dir::E => 'E',
                Dir::S => 'S',
                Dir::W => 'N',
            }
        )
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bit_set_iter() {
        let mut b = BitSet::new(10);
        b.insert(3);
        b.insert(7);
        let mut iter = b.iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(7));
        assert_eq!(iter.next(), None);
    }
}
