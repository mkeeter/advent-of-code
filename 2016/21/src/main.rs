use std::io::BufRead;
use std::str::FromStr;

////////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Debug)]
enum Action {
    SwapPosition(usize, usize),
    SwapLetter(char, char),
    RotateLeft(usize),
    RotateRight(usize),
    RotateLetter(char),
    Reverse(usize, usize),
    Move(usize, usize),
}
use Action::*;

impl FromStr for Action {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        let words = s.split(' ').collect::<Vec<_>>();
        match words[0] {
            "swap" => match words[1] {
                "position" => Ok(SwapPosition(
                    usize::from_str(words[2]).unwrap(),
                    usize::from_str(words[5]).unwrap())),
                "letter" => Ok(SwapLetter(
                    words[2].chars().next().unwrap(),
                    words[5].chars().next().unwrap())),
                _ => Err(()),
            }
            "rotate" => match words[1] {
                "left" => Ok(RotateLeft(
                    usize::from_str(words[2]).unwrap())),
                "right" => Ok(RotateRight(
                    usize::from_str(words[2]).unwrap())),
                "based" => Ok(RotateLetter(
                    words[6].chars().next().unwrap())),
                _ => Err(()),
            },
            "reverse" => Ok(Reverse(
                usize::from_str(words[2]).unwrap(),
                usize::from_str(words[4]).unwrap())),
            "move" => Ok(Move(
                usize::from_str(words[2]).unwrap(),
                usize::from_str(words[5]).unwrap())),
            _ => Err(()),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
struct Word(Vec<char>);

impl Word {
    fn find(&self, c: char) -> usize {
        self.0.iter()
            .enumerate()
            .find(|a| *a.1 == c)
            .unwrap().0
    }
    fn apply(&mut self, a: Action) {
        match a {
            SwapPosition(ia, ib) => {
                let ca = self.0[ia];
                let cb = self.0[ib];
                self.0[ia] = cb;
                self.0[ib] = ca;
            },
            SwapLetter(ca, cb) => {
                let ia = self.find(ca);
                let ib = self.find(cb);
                self.0[ia] = cb;
                self.0[ib] = ca;
            },
            RotateLeft(n) => {
                self.0.rotate_left(n);
            },
            RotateRight(n) => {
                self.0.rotate_right(n);
            },
            RotateLetter(c) => {
                let i = self.find(c);
                let rot = (1 + i + (i >= 4) as usize) % self.0.len();
                self.0.rotate_right(rot);
            },
            Reverse(ia, ib) => {
                for i in 0..((ib - ia + 1) / 2) {
                    let ca = self.0[ia + i];
                    let cb = self.0[ib - i];
                    self.0[ia + i] = cb;
                    self.0[ib - i] = ca;
                }
            }
            Move(ia, ib) => {
                let c = self.0.remove(ia);
                self.0.insert(ib, c);
            }
        }
    }
}

fn main() {
    let input = std::io::stdin().lock()
        .lines()
        .filter_map(|line| Action::from_str(&line.unwrap()).ok())
        .collect::<Vec<Action>>();

    let mut word = Word("abcdefgh".chars().collect());
    for a in input.into_iter() {
        word.apply(a);
    }
    println!("Part 1: {}", word.0.iter().collect::<String>());
}
