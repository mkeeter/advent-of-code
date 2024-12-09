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
enum Data {
    File { index: usize },
    Gap,
}

#[derive(Copy, Clone, Debug)]
struct Span {
    data: Data,
    length: usize,
}

/// Tries to pack the last file into the `data` tree
///
/// Returns the file if there are no gaps that can fit it, otherwise, returns
/// `None` and moves the file to the leftmost gap of appropriate size.
fn pack_one_file(data: &mut BTreeMap<usize, Span>) -> Option<(usize, Span)> {
    // Remove the last file from the tree
    let (index, f) = loop {
        let (i, last) = data.pop_last().unwrap();
        if matches!(last.data, Data::Gap) {
            continue;
        } else {
            break (i, last);
        }
    };
    println!("packing {f:?}");
    if let Some((i, d)) = data
        .iter_mut()
        .find(|(_i, d)| matches!(d.data, Data::Gap) && d.length >= f.length)
    {
        println!("found gap at {i}, {d:?}");
        let new_gap_length = d.length - f.length;
        let new_gap_pos = i + f.length;
        *d = f; // put the file at the beginning of the gap
        println!("inserting {new_gap_length} gap at {new_gap_pos}");
        data.insert(
            new_gap_pos,
            Span {
                data: Data::Gap,
                length: new_gap_length,
            },
        );
        None
    } else {
        println!("did not find gap");
        Some((index, f))
    }
}

fn pack_files(mut data: BTreeMap<usize, Span>) -> usize {
    let mut out = vec![];
    while !data.is_empty() {
        out.extend(pack_one_file(&mut data));
    }
    let mut checksum = 0;
    for &(pos, v) in out.iter().rev() {
        let Data::File { index } = v.data else {
            panic!();
        };
        for i in pos..pos + v.length {
            checksum += index * i;
        }
    }
    checksum
}

pub fn solve(s: &str) -> (usize, usize) {
    let mut data = vec![];
    let mut tree = BTreeMap::new();
    let mut pos = 0;
    for (i, c) in s.chars().enumerate() {
        if c.is_ascii_digit() {
            let index = if i % 2 == 0 { Some(i / 2) } else { None };
            let length = (c as u8 - b'0') as usize;
            for _ in 0..length {
                data.push(if i % 2 == 0 { Some(i / 2) } else { None });
            }
            tree.insert(
                pos,
                Span {
                    data: index
                        .map(|index| Data::File { index })
                        .unwrap_or(Data::Gap),
                    length,
                },
            );
            pos += length;
        }
    }
    (pack_blocks(&data), pack_files(tree))
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
