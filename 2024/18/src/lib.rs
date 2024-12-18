use std::collections::VecDeque;
use util::{get_integers, Dir, TupleSet};

fn search(blocked: &[bool], map_size: usize) -> Option<u64> {
    let mut todo = VecDeque::new();
    todo.push_back((0i64, 0i64, 0));
    let mut seen = TupleSet::new((map_size, map_size));
    while let Some((x, y, t)) = todo.pop_front() {
        if !seen.insert((x as usize, y as usize)) {
            continue;
        }
        if x == map_size as i64 - 1 && y == map_size as i64 - 1 {
            return Some(t);
        }
        for d in Dir::iter() {
            let (nx, ny) = (x + d.x(), y + d.y());
            if nx >= 0
                && nx < map_size as i64
                && ny >= 0
                && ny < map_size as i64
                && !blocked[(nx + ny * map_size as i64) as usize]
            {
                todo.push_back((nx, ny, t + 1));
            }
        }
    }
    None
}

fn run(s: &str, map_size: usize, p1_time: u32) -> (u64, String) {
    let mut blocked = vec![false; (map_size + 1).pow(2)];
    let mut iter = get_integers::<usize>(s);

    for _ in 0..p1_time {
        let x = iter.next().unwrap();
        let y = iter.next().unwrap();
        let i = x + y * map_size;
        blocked[i] = true;
    }

    let p1 = search(&blocked, map_size).unwrap();
    let p2 = loop {
        let x = iter.next().unwrap();
        let y = iter.next().unwrap();
        let i = x + y * map_size;
        blocked[i] = true;
        if search(&blocked, map_size).is_none() {
            break format!("{x},{y}");
        }
    };

    (p1, p2)
}

pub fn solve(s: &str) -> (u64, String) {
    run(s, 71, 1024)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn example() {
        const EXAMPLE: &str = indoc::indoc! {"
            5,4
            4,2
            4,5
            3,0
            2,1
            6,3
            2,4
            1,5
            0,6
            3,3
            2,6
            5,1
            1,2
            5,5
            2,5
            6,5
            1,4
            0,4
            6,4
            1,1
            6,1
            1,0
            0,5
            1,6
            2,0
        "};
        assert_eq!(run(EXAMPLE, 7, 12), (22, "6,1".to_string()));
    }
}
