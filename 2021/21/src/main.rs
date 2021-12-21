use std::io::BufRead;

struct Player {
    pos: u64,
    score: u64,
}

fn main() {
    let stdin = std::io::stdin();
    let mut iter = stdin
        .lock()
        .lines()
        .map(|line| line.unwrap().bytes().last().unwrap() - b'0');

    let mut a = Player { pos: iter.next().unwrap() as u64, score: 0 };
    let mut b = Player { pos: iter.next().unwrap() as u64, score: 0 };

    let mut roll = 1;
    loop {
        for _ in 0..3 {
            a.pos = ((a.pos + ((roll - 1) % 100)) % 10) + 1;
            roll += 1;
        }
        a.score += a.pos;
        if a.score >= 1000 {
            println!("Part 1: {}", b.score * (roll - 1));
            break;
        }

        for _ in 0..3 {
            b.pos = ((b.pos + ((roll - 1) % 100)) % 10) + 1;
            roll += 1;
        }
        b.score += b.pos;
        if b.score >= 1000 {
            println!("Part 1: {}", a.score * (roll - 1));
            break;
        }
    }
}
