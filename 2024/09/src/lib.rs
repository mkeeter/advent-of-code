use std::collections::BTreeMap;

fn pack_blocks(data: &[Option<u16>]) -> u64 {
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
    data.iter()
        .enumerate()
        .map(|(i, v)| i as u64 * u64::from(v.unwrap_or(0)))
        .sum()
}

#[derive(Copy, Clone, Debug)]
struct File {
    index: u16,
    length: u8,
}

impl File {
    fn length(&self) -> usize {
        usize::from(self.length)
    }
}

#[derive(Copy, Clone, Debug)]
struct Gap {
    length: usize,
}

#[derive(Default)]
struct GapTree(BTreeMap<usize, Gap>);

impl GapTree {
    fn find_space_for(&mut self, f: File) -> Option<usize> {
        if let Some((&i, ref d)) =
            self.0.iter_mut().find(|(_i, g)| g.length >= f.length())
        {
            let new_gap_length = d.length - f.length();
            let new_gap_pos = i + f.length();
            self.0.remove(&i);
            if new_gap_length > 0 {
                self.0.insert(
                    new_gap_pos,
                    Gap {
                        length: new_gap_length,
                    },
                );
            }
            Some(i)
        } else {
            None
        }
    }
    fn trim(&mut self, index: usize) {
        while let Some((i, g)) = self.0.last_key_value() {
            if i + g.length > index {
                self.0.pop_last();
            } else {
                break;
            }
        }
    }
}

#[derive(Default)]
struct FileTree(BTreeMap<usize, File>);

impl FileTree {
    fn pop_last(&mut self) -> Option<(usize, File)> {
        self.0.pop_last()
    }
    fn insert(&mut self, index: usize, f: File) {
        self.0.insert(index, f);
    }
}

fn pack_files(mut files: FileTree, mut gaps: GapTree) -> u64 {
    let mut out = BTreeMap::new();
    while let Some((index, f)) = files.pop_last() {
        gaps.trim(index);
        let i = gaps.find_space_for(f).unwrap_or(index);
        out.insert(i, f);
    }
    let mut checksum = 0;
    for (&pos, v) in out.iter().rev() {
        for i in pos..pos + v.length() {
            checksum += v.index as u64 * i as u64;
        }
    }
    checksum
}

pub fn solve(s: &str) -> (u64, u64) {
    let mut data = vec![];
    let mut files = FileTree::default();
    let mut gaps = GapTree::default();
    let mut pos = 0;
    for (i, c) in s.chars().enumerate() {
        if c.is_ascii_digit() {
            let index = if i % 2 == 0 {
                Some(u16::try_from(i / 2).unwrap())
            } else {
                None
            };
            let length = c as u8 - b'0';
            for _ in 0..length {
                data.push(index);
            }
            if let Some(index) = index {
                files.insert(pos, File { index, length })
            } else if length > 0 {
                gaps.0.insert(
                    pos,
                    Gap {
                        length: usize::from(length),
                    },
                );
            }
            pos += usize::from(length);
        }
    }
    (pack_blocks(&data), pack_files(files, gaps))
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn example() {
        const EXAMPLE: &str = "2333133121414131402";
        assert_eq!(solve(EXAMPLE), (1928, 2858));
    }
}
