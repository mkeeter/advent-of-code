#[derive(Clone)]
struct SkipBuf<'a> {
    data: &'a [i8],
    skip: usize,
}

impl std::ops::Index<usize> for SkipBuf<'_> {
    type Output = i8;
    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.skip {
            &self.data[index + 1]
        } else {
            &self.data[index]
        }
    }
}

impl SkipBuf<'_> {
    fn len(&self) -> usize {
        self.data.len() - 1
    }
}

impl<'a, 'b> IntoIterator for &'a SkipBuf<'b>
where
    'a: 'b,
{
    type Item = &'b i8;
    type IntoIter = SkipBufIter<'a, 'b>;
    fn into_iter(self) -> Self::IntoIter {
        SkipBufIter {
            buf: self,
            index: 0,
        }
    }
}

struct SkipBufIter<'a, 'b> {
    buf: &'a SkipBuf<'b>,
    index: usize,
}

impl<'a, 'b> Iterator for SkipBufIter<'a, 'b>
where
    'a: 'b,
{
    type Item = &'b i8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.buf.len() {
            let out = &self.buf[self.index];
            self.index += 1;
            Some(out)
        } else {
            None
        }
    }
}

fn is_safe<'a, T>(row: &'a T) -> bool
where
    T: std::ops::Index<usize, Output = i8>,
    &'a T: IntoIterator<Item = &'a i8>,
{
    let sign = (row[1] - row[0]).signum();
    if sign == 0 {
        return false;
    }

    row.into_iter().zip(row.into_iter().skip(1)).all(|(a, b)| {
        let d = *b - *a;
        d.signum() == sign && d.abs() <= 3
    })
}

fn any_safe(row: &[i8]) -> bool {
    (0..row.len()).any(|skip| is_safe(&SkipBuf { data: row, skip }))
}

pub fn solve(s: &str) -> (usize, usize) {
    let mut rows = vec![];
    for line in s.lines() {
        rows.push(
            line.split_ascii_whitespace()
                .map(|i| i.parse::<i8>().unwrap())
                .collect::<Vec<_>>(),
        );
    }

    let safe = rows.iter().filter(|row| is_safe(*row)).count();
    let modified = rows.iter().filter(|row| any_safe(row)).count();

    (safe, modified)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let e = indoc::indoc! {"
            7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9
        "};
        let (a, b) = solve(e);
        assert_eq!(a, 2);
        assert_eq!(b, 4);
    }
}
