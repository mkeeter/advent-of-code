type Map = Vec<Vec<bool>>;

fn is_symmetric_about(row: &[bool], pos: usize) -> bool {
    for i in 0..=pos {
        let far = (pos - i) + pos + 1;
        if let Some(p) = row.get(far) {
            if *p != row[i] {
                return false;
            }
        }
    }
    true
}

fn transpose(map: &Map) -> Map {
    let mut out = vec![];
    for col in 0..map[0].len() {
        let row = (0..map.len()).map(|y| map[y][col]).collect();
        out.push(row);
    }
    out
}

fn find_mirror(map: &Map) -> Option<usize> {
    let mut valid: Vec<usize> = (0..map[0].len() - 1).collect();
    for row in map {
        valid.retain(|v| is_symmetric_about(row, *v));
    }
    if valid.len() == 1 {
        Some(valid[0] + 1)
    } else {
        assert!(valid.is_empty());
        None
    }
}

fn find_smudged_mirror(map: &Map) -> Option<usize> {
    // Use a 32-bit word as a simple set, with one bit per row
    assert!(map.len() < 32);
    let mut valid = vec![0u32; map[0].len() - 1];
    for (r, row) in map.iter().enumerate() {
        for (i, v) in valid.iter_mut().enumerate() {
            if !is_symmetric_about(row, i) {
                *v |= 1 << r;
            }
        }
    }
    for (i, r) in valid
        .iter()
        .enumerate()
        .filter(|(_i, v)| v.count_ones() == 1)
        .map(|(i, v)| (i, v.trailing_zeros() as usize))
    {
        let mut row = map[r].to_vec();
        for j in 0..row.len() {
            row[j] = !row[j];
            if is_symmetric_about(&row, i) {
                return Some(i + 1);
            }
            row[j] = !row[j];
        }
    }
    None
}

pub fn solve(s: &str) -> (String, String) {
    let mut hmaps: Vec<Map> = vec![vec![]];
    for line in s.lines() {
        if line.is_empty() {
            hmaps.push(vec![]);
        } else {
            let row = line.chars().map(|c| c == '#').collect();
            hmaps.last_mut().unwrap().push(row);
        }
    }
    let vmaps: Vec<Map> = hmaps.iter().map(transpose).collect();

    let mut out = 0;
    for (hmap, vmap) in hmaps.iter().zip(vmaps.iter()) {
        if let Some(h) = find_mirror(hmap) {
            out += h;
        } else if let Some(v) = find_mirror(vmap) {
            out += v * 100;
        } else {
            panic!("no mirror found");
        }
    }
    let p1 = out;

    let mut out = 0;
    for (hmap, vmap) in hmaps.iter().zip(vmaps.iter()) {
        if let Some(h) = find_smudged_mirror(hmap) {
            out += h;
        } else if let Some(v) = find_smudged_mirror(vmap) {
            out += v * 100;
        } else {
            panic!("no smudge found");
        }
    }
    let p2 = out;

    (p1.to_string(), p2.to_string())
}
