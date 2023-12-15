fn hash(s: &str) -> u8 {
    let mut hash = 0u8;
    for c in s.bytes() {
        hash = hash.wrapping_add(c);
        hash = hash.wrapping_mul(17);
    }
    hash
}

pub fn solve(s: &str) -> (String, String) {
    let s = s.trim();

    let p1: u64 = s.split(',').map(|s| hash(s) as u64).sum();

    let mut boxes: [Vec<(&str, u8)>; 256] = [(); 256].map(|_| vec![]);
    for cmd in s.split(',') {
        if let Some(label) = cmd.strip_suffix('-') {
            let i = hash(label) as usize;
            boxes[i].retain(|(l, _)| *l != label);
        } else {
            let mut iter = cmd.split('=');
            let label = iter.next().unwrap();
            let i = hash(label) as usize;
            let focal_length = iter.next().unwrap().parse::<u8>().unwrap();
            if let Some(prev) = boxes[i].iter_mut().find(|(l, _)| *l == label) {
                prev.1 = focal_length;
            } else {
                boxes[i].push((label, focal_length));
            }
        }
    }
    let mut p2 = 0;
    for (i, b) in boxes.iter().enumerate() {
        for (j, &(_label, focal_length)) in b.iter().enumerate() {
            let score = (i + 1) * (j + 1) * focal_length as usize;
            p2 += score;
        }
    }

    (p1.to_string(), p2.to_string())
}
