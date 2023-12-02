use anyhow::Result;
use std::io::BufRead;

#[derive(Debug)]
struct Game {
    index: usize,
    rgb: Vec<[usize; 3]>,
}

impl Game {
    fn max_seen(&self) -> [usize; 3] {
        let mut out = [0; 3];
        for v in &self.rgb {
            for i in 0..3 {
                out[i] = out[i].max(v[i]);
            }
        }
        out
    }
}

fn main() -> Result<()> {
    let lines = std::io::stdin()
        .lock()
        .lines()
        .collect::<Result<Vec<String>, _>>()?;

    let mut games = vec![];
    for line in lines {
        let mut iter = line.split(": ");
        let mut game = iter.next().unwrap().split(' ');
        assert_eq!(game.next().unwrap(), "Game");
        let index = game.next().unwrap().parse::<usize>().unwrap();

        let mut rgb = vec![];
        for r in iter.next().unwrap().split("; ") {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
            for ball in r.split(", ") {
                let mut iter = ball.split(' ');
                let count = iter.next().unwrap().parse::<usize>()?;
                *match iter.next().unwrap() {
                    "red" => &mut red,
                    "green" => &mut green,
                    "blue" => &mut blue,
                    c => panic!("invalid color {c}"),
                } = count;
            }
            rgb.push([red, green, blue]);
        }
        games.push(Game { index, rgb });
    }

    let mut out = 0;
    for game in &games {
        let [r, g, b] = game.max_seen();
        if r <= 12 && g <= 13 && b <= 14 {
            out += game.index;
        }
    }
    println!("Part 1: {out}");

    let mut out = 0;
    for game in &games {
        let [r, g, b] = game.max_seen();
        out += r * g * b;
    }
    println!("Part 2: {out}");

    Ok(())
}
