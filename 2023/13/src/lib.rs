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
    let mut valid = vec![0; map[0].len() - 1];
    for row in map {
        for (i, v) in valid.iter_mut().enumerate() {
            if is_symmetric_about(row, i) {
                *v += 1;
            }
        }
    }
    for i in valid
        .iter()
        .enumerate()
        .filter(|(_i, v)| **v == map.len() - 1)
        .map(|(i, _v)| i)
    {
        for row in map.iter().filter(|row| !is_symmetric_about(row, i)) {
            let mut row = row.to_vec();
            for j in 0..row.len() {
                row[j] = !row[j];
                if is_symmetric_about(&row, i) {
                    return Some(i + 1);
                }
                row[j] = !row[j];
            }
        }
    }
    None
}

pub fn solve(s: &str) -> (String, String) {
    let mut maps: Vec<Map> = vec![vec![]];
    for line in s.lines() {
        if line.is_empty() {
            maps.push(vec![]);
        } else {
            let row = line.chars().map(|c| c == '#').collect();
            maps.last_mut().unwrap().push(row);
        }
    }

    let mut out = 0;
    for map in &maps {
        if let Some(h) = find_mirror(map) {
            out += h;
        } else if let Some(v) = find_mirror(&transpose(map)) {
            out += v * 100;
        } else {
            panic!("no mirror found");
        }
    }
    let p1 = out;

    let mut out = 0;
    for map in &maps {
        if let Some(h) = find_smudged_mirror(map) {
            out += h;
        } else if let Some(v) = find_smudged_mirror(&transpose(map)) {
            out += v * 100;
        } else {
            panic!("no smudge found");
        }
    }
    let p2 = out;

    (p1.to_string(), p2.to_string())
}
