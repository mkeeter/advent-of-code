struct Position {
    index: usize,
    remaining: usize,
}

struct PackIter<'a> {
    bytes: &'a [u8],
    forward: Position,
    reverse: Position,
}

struct RepeatedValue {
    value: usize,
    count: usize,
}

impl Iterator for PackIter<'_> {
    type Item = RepeatedValue; // tuple of (value, count)
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

fn checksum(data: &[Option<usize>]) -> usize {
    data.iter()
        .enumerate()
        .map(|(i, v)| i * v.unwrap_or(0))
        .sum()
}

fn pack_blocks(data: &[Option<usize>]) -> usize {
    let mut forward = 0;
    let mut reverse = data.len() - 1;
    let mut data = data.to_vec();
    while forward < reverse {
        while data[forward].is_some() {
            forward += 1;
        }
        while data[reverse].is_none() {
            reverse -= 1;
        }
        while forward < reverse
            && data[forward].is_none()
            && data[reverse].is_some()
        {
            data.swap(forward, reverse);
            forward += 1;
            reverse -= 1;
        }
    }
    checksum(&data)
}

enum Data {
    File { index: usize },
    Gap,
}

struct Span {
    data: Data,
    length: usize,
}

fn pack_files(data: &[Option<usize>]) -> usize {
    let mut forward = 0;
    let mut reverse = data.len() - 1;
    let mut data = data.to_vec();
    while forward < reverse {
        while data[forward].is_some() {
            forward += 1;
        }
        while data[reverse].is_none() {
            reverse -= 1;
        }
        while forward < reverse
            && data[forward].is_none()
            && data[reverse].is_some()
        {
            data.swap(forward, reverse);
            forward += 1;
            reverse -= 1;
        }
    }
    checksum(&data)
}

pub fn solve(s: &str) -> (usize, u64) {
    let mut data = vec![];
    for (i, c) in s.chars().enumerate() {
        if c.is_ascii_digit() {
            for _ in 0..(c as u8 - b'0') {
                data.push(if i % 2 == 0 { Some(i / 2) } else { None });
            }
        }
    }
    (pack_blocks(&data), 0)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn example() {
        const EXAMPLE: &str = "2333133121414131402";
        assert_eq!(solve(EXAMPLE), (1928, 0));
    }
}
