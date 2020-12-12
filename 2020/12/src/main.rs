use std::io::BufRead;

type State = ((i64, i64), (i64, i64));
fn run<F>(input: &Vec<(char, i64)>, state: State, f: F) -> i64
    where F: Fn(char, State) -> State
{
    let out = input.iter().fold(state, |state, (cmd, count)| {
        let count = count / match cmd {
            'R'|'L' => 90,
            _ => 1,
        };
        std::iter::successors(Some(state), |state| Some(f(*cmd, *state)))
            .nth(count as usize)
            .unwrap()
    });
    out.0.0.abs() + out.0.1.abs()
}

fn main() {
    let input: Vec<(char, i64)> = std::io::stdin().lock().lines()
        .map(|line| {
            let line = line.unwrap();
            let (cmd, count) = line.split_at(1);
            (cmd.chars().next().unwrap(), count.parse().unwrap())
        })
        .collect();

    let p1 = |cmd, (pos, dir): State| match cmd {
        'N' => ((pos.0, pos.1 + 1), dir),
        'S' => ((pos.0, pos.1 - 1), dir),
        'E' => ((pos.0 + 1, pos.1), dir),
        'W' => ((pos.0 - 1, pos.1), dir),
        'F' => ((pos.0 + dir.0, pos.1 + dir.1), dir),
        'L' => (pos, (-dir.1,  dir.0)),
        'R' => (pos, ( dir.1, -dir.0)),
        _ => panic!("Invalid command {}", cmd),
    };
    println!("Part 1: {}", run(&input, ((0, 0), (1, 0)), p1));

    let p2 = |cmd, (pos, dir): State| match cmd {
        'N' => (pos, (dir.0, dir.1 + 1)),
        'S' => (pos, (dir.0, dir.1 - 1)),
        'E' => (pos, (dir.0 + 1, dir.1)),
        'W' => (pos, (dir.0 - 1, dir.1)),
        'F' => ((pos.0 + dir.0, pos.1 + dir.1), dir),
        'L' => (pos, (-dir.1,  dir.0)),
        'R' => (pos, ( dir.1, -dir.0)),
        _ => panic!("Invalid command {}", cmd),
    };
    println!("Part 2: {}", run(&input, ((0, 0), (10, 1)), p2));
}
