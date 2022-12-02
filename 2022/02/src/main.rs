use anyhow::{anyhow, bail, Error, Result};
use std::io::BufRead;

#[derive(Copy, Clone, Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl std::str::FromStr for Move {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            c => bail!("Invalid character '{c}'"),
        }
    }
}

impl Move {
    fn score(&self) -> usize {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
    fn play(&self, opponent: Move) -> usize {
        match (self, opponent) {
            // Win
            (Move::Rock, Move::Scissors)
            | (Move::Scissors, Move::Paper)
            | (Move::Paper, Move::Rock) => 6,
            // Lose
            (Move::Scissors, Move::Rock)
            | (Move::Paper, Move::Scissors)
            | (Move::Rock, Move::Paper) => 0,
            // Draw
            (Move::Scissors, Move::Scissors)
            | (Move::Paper, Move::Paper)
            | (Move::Rock, Move::Rock) => 3,
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Goal {
    Win,
    Lose,
    Draw,
}

impl std::str::FromStr for Goal {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "X" => Ok(Goal::Lose),
            "Y" => Ok(Goal::Draw),
            "Z" => Ok(Goal::Win),
            c => bail!("Invalid character '{c}'"),
        }
    }
}

impl Goal {
    fn play(&self, opponent: Move) -> Move {
        match (self, opponent) {
            (Goal::Win, Move::Rock)
            | (Goal::Draw, Move::Paper)
            | (Goal::Lose, Move::Scissors) => Move::Paper,

            (Goal::Win, Move::Paper)
            | (Goal::Draw, Move::Scissors)
            | (Goal::Lose, Move::Rock) => Move::Scissors,

            (Goal::Win, Move::Scissors)
            | (Goal::Draw, Move::Rock)
            | (Goal::Lose, Move::Paper) => Move::Rock,
        }
    }
}

fn score(plays: &[(Move, Move)]) -> usize {
    plays
        .iter()
        .map(|(opponent, you)| you.score() + you.play(*opponent))
        .sum()
}

fn main() -> Result<()> {
    let lines = std::io::stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .collect::<Vec<String>>();

    let plays = lines
        .iter()
        .map(|line| -> Result<(Move, Move)> {
            let out = line
                .split(' ')
                .map(str::parse)
                .collect::<Result<Vec<Move>, _>>()?;
            if out.len() != 2 {
                bail!("Unexpected line: {out:?}");
            }
            Ok((out[0], out[1]))
        })
        .collect::<Result<Vec<(Move, Move)>, _>>()?;
    println!("Part 1: {}", score(&plays));

    let plays = lines
        .iter()
        .map(|line| -> Result<(Move, Move)> {
            let mut out = line.split(' ');
            let opponent = out
                .next()
                .ok_or_else(|| anyhow!("Missing command"))
                .and_then(str::parse)?;
            let goal: Goal = out
                .next()
                .ok_or_else(|| anyhow!("Missing command"))
                .and_then(str::parse)?;
            if let Some(e) = out.next() {
                bail!("Unexpected value after second char: {e}");
            }
            Ok((opponent, goal.play(opponent)))
        })
        .collect::<Result<Vec<(Move, Move)>, _>>()?;
    println!("Part 2: {}", score(&plays));

    Ok(())
}
