use std::collections::HashMap;
use std::io::BufRead;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Player {
    pos: u64,
    score: u64,
}

fn deterministic(mut a: Player, mut b: Player) -> u64 {
    let mut roll = 1;
    loop {
        for _ in 0..3 {
            a.pos = ((a.pos + ((roll - 1) % 100)) % 10) + 1;
            roll += 1;
        }
        a.score += a.pos;
        if a.score >= 1000 {
            return b.score * (roll - 1);
        }

        for _ in 0..3 {
            b.pos = ((b.pos + ((roll - 1) % 100)) % 10) + 1;
            roll += 1;
        }
        b.score += b.pos;
        if b.score >= 1000 {
            return a.score * (roll - 1);
        }
    }
}

fn dirac(
    a: Player,
    b: Player,
    turn: bool,
    cache: &mut HashMap<(Player, Player, bool), (u64, u64)>,
) -> (u64, u64) {
    if a.score >= 21 {
        return (1, 0);
    } else if b.score >= 21 {
        return (0, 1);
    } else if let Some(out) = cache.get(&(a, b, turn)) {
        return *out;
    }
    let mut out = (0, 0);
    for i in 1..=3 {
        for j in 1..=3 {
            for k in 1..=3 {
                let mut a = a;
                let mut b = b;
                if turn {
                    a.pos = ((a.pos + i + j + k - 1) % 10) + 1;
                    a.score += a.pos;
                } else {
                    b.pos = ((b.pos + i + j + k - 1) % 10) + 1;
                    b.score += b.pos;
                }
                let r = dirac(a, b, !turn, cache);
                out.0 += r.0;
                out.1 += r.1;
            }
        }
    }
    cache.insert((a, b, turn), out);
    out
}

fn main() {
    let stdin = std::io::stdin();
    let mut iter = stdin
        .lock()
        .lines()
        .map(|line| line.unwrap().bytes().last().unwrap() - b'0');

    let a = Player {
        pos: iter.next().unwrap() as u64,
        score: 0,
    };
    let b = Player {
        pos: iter.next().unwrap() as u64,
        score: 0,
    };

    println!("Part 1: {}", deterministic(a, b));

    let mut cache = HashMap::new();
    let out = dirac(a, b, true, &mut cache);
    println!("Part 2: {:?}", out.0.max(out.1));
}
