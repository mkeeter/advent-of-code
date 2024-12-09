use std::collections::BTreeMap;

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
    data.iter()
        .enumerate()
        .map(|(i, v)| i * v.unwrap_or(0))
        .sum()
}

#[derive(Copy, Clone, Debug)]
struct File {
    index: usize,
    length: usize,
}

struct Gap {
    length: usize,
}

#[derive(Default)]
struct GapTree(BTreeMap<usize, Gap>);

impl GapTree {
    fn find_space_for(&mut self, f: File) -> Option<usize> {
        if let Some((&i, ref d)) =
            self.0.iter_mut().find(|(_i, g)| g.length >= f.length)
        {
            let new_gap_length = d.length - f.length;
            let new_gap_pos = i + f.length;
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
    fn insert(&mut self, index: usize, g: Gap) {
        let end = index + g.length;
        // Merge gaps forwards
        if let Some(v) = self.0.remove(&end) {
            self.0.insert(
                index,
                Gap {
                    length: g.length + v.length,
                },
            );
        } else {
            // Merge gaps in reverse
            let mut prev = self.0.range_mut(..index);
            match prev.next_back() {
                Some((i, p)) if i + p.length == index => p.length += g.length,
                _ => {
                    self.0.insert(index, g);
                }
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
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// Tries to pack the last file into the `data` tree
///
/// Returns the file if there are no gaps that can fit it, otherwise, returns
/// `None` and moves the file to the leftmost gap of appropriate size.
fn pack_one_file(
    files: &mut FileTree,
    gaps: &mut GapTree,
) -> Option<(usize, File)> {
    let (index, f) = files.pop_last().unwrap();
    if let Some(i) = gaps.find_space_for(f) {
        if i >= index {
            return Some((index, f));
        }
        files.insert(i, f);
        gaps.insert(index, Gap { length: f.length });
        None
    } else {
        Some((index, f))
    }
}

fn pack_files(mut files: FileTree, mut gaps: GapTree) -> usize {
    let mut out = vec![];
    while !files.is_empty() {
        out.extend(pack_one_file(&mut files, &mut gaps));
    }
    let mut checksum = 0;
    for &(pos, v) in out.iter().rev() {
        for i in pos..pos + v.length {
            checksum += v.index * i;
        }
    }
    checksum
}

pub fn solve(s: &str) -> (usize, usize) {
    let mut data = vec![];
    let mut files = FileTree::default();
    let mut gaps = GapTree::default();
    let mut pos = 0;
    for (i, c) in s.chars().enumerate() {
        if c.is_ascii_digit() {
            let index = if i % 2 == 0 { Some(i / 2) } else { None };
            let length = (c as u8 - b'0') as usize;
            for _ in 0..length {
                data.push(if i % 2 == 0 { Some(i / 2) } else { None });
            }
            if let Some(index) = index {
                files.insert(pos, File { index, length })
            } else {
                gaps.insert(pos, Gap { length })
            }
            pos += length;
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
