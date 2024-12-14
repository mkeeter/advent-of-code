use util::GridSet;

const WIDTH: i64 = 101;
const HEIGHT: i64 = 103;

#[derive(Copy, Clone)]
struct Bot {
    x: i64,
    y: i64,
    dx: i64,
    dy: i64,
}

impl Bot {
    fn run(&self, steps: i64, width: i64, height: i64) -> Bot {
        let x = (self.x + self.dx * steps).rem_euclid(width);
        let y = (self.y + self.dy * steps).rem_euclid(height);
        Bot { x, y, ..*self }
    }
}

fn get_quads(bots: &[Bot], width: i64, height: i64) -> usize {
    let mut quads = [0; 4];
    for bot in bots {
        const STEPS: i64 = 100;
        let b = bot.run(STEPS, width, height);
        if b.x != width / 2 && b.y != height / 2 {
            quads[(b.x <= width / 2) as usize * 2
                + (b.y <= height / 2) as usize] += 1;
        }
    }
    quads.iter().product()
}

fn score(bots: &[Bot], seen: &GridSet) -> usize {
    let mut score = 0;
    for b in bots.iter() {
        for dx in [-1, 0, 1] {
            for dy in [-1, 0, 1] {
                let x = b.x + dx;
                let y = b.y + dy;
                if (0..WIDTH).contains(&x)
                    && (0..HEIGHT).contains(&y)
                    && !(x == 0 && y == 0)
                    && seen.contains(x, y)
                {
                    score += 1;
                }
            }
        }
    }
    score
}

fn find_tree(bots: &mut [Bot]) -> usize {
    let mut seen = GridSet::from_width_and_height(WIDTH, HEIGHT);
    for i in 1.. {
        seen.clear();
        for b in bots.iter_mut() {
            *b = b.run(1, WIDTH, HEIGHT);
            seen.insert(b.x, b.y);
        }
        if score(bots, &seen) > 1000 {
            return i;
        }
    }
    unreachable!()
}

fn parse(line: &str) -> Bot {
    let mut chunks = line.split(' ');
    let pos = chunks.next().unwrap();
    let mut iter = pos[2..].split(',');
    let x = iter.next().and_then(|s| s.parse::<i64>().ok()).unwrap();
    let y = iter.next().and_then(|s| s.parse::<i64>().ok()).unwrap();

    let vel = chunks.next().unwrap();
    let mut iter = vel[2..].split(',');
    let dx = iter.next().and_then(|s| s.parse::<i64>().ok()).unwrap();
    let dy = iter.next().and_then(|s| s.parse::<i64>().ok()).unwrap();

    Bot { x, y, dx, dy }
}

pub fn solve(s: &str) -> (usize, usize) {
    let mut bots = s.lines().map(parse).collect::<Vec<Bot>>();

    (get_quads(&bots, WIDTH, HEIGHT), find_tree(&mut bots))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        const EXAMPLE: &str = indoc::indoc! {"
            p=0,4 v=3,-3
            p=6,3 v=-1,-3
            p=10,3 v=-1,2
            p=2,0 v=2,-1
            p=0,0 v=1,3
            p=3,0 v=-2,-2
            p=7,6 v=-1,-3
            p=3,0 v=-1,-2
            p=9,3 v=2,3
            p=7,3 v=-1,2
            p=2,4 v=2,-3
            p=9,5 v=-3,-3
        "};
        let bots = EXAMPLE.lines().map(parse).collect::<Vec<Bot>>();
        assert_eq!(get_quads(&bots, 11, 7), 12);
    }
}
