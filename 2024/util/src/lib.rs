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
