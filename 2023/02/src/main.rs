use anyhow::Result;
use std::io::BufRead;

type Rgb = [usize; 3];

#[derive(Debug)]
struct Game {
    index: usize,
    rgb: Vec<Rgb>,
}

impl Game {
    fn max_seen(&self) -> Rgb {
        let mut out = [0; 3];
        for v in &self.rgb {
            for i in 0..out.len() {
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
        let mut game = iter.next().unwrap().split_whitespace();
        assert_eq!(game.next().unwrap(), "Game");
        let index = game.next().unwrap().parse::<usize>().unwrap();

        let mut rgb = vec![];
        for r in iter.next().unwrap().split("; ") {
            let mut v = Rgb::default();
            for ball in r.split(", ") {
                let mut iter = ball.split_whitespace();
                let count = iter.next().unwrap().parse::<usize>()?;
                let color = iter.next().unwrap();
                let i = ["red", "green", "blue"]
                    .iter()
                    .position(|s| *s == color)
                    .unwrap();
                assert_eq!(v[i], 0);
                v[i] = count;
            }
            rgb.push(v);
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
