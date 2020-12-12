use std::io::BufRead;

type State = ((i64, i64), (i64, i64)); // (pos, dir or waypoint)

fn run<F>(input: &[(char, i64)], state: State, f: F) -> i64
    where F: Fn(State, char) -> State
{
    let out = input.iter()
        // Expand [command x count] into a sequence of repeated commands
        .flat_map(|&(cmd, count)| std::iter::repeat(cmd).take(
            match cmd {
                'R'|'L' => count / 90,
                _ => count,
            } as usize))
        .fold(state, f);
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

    let common = |(pos, dir): State, cmd| match cmd {
        'F' => ((pos.0 + dir.0, pos.1 + dir.1), dir),
        'L' => (pos, (-dir.1,  dir.0)),
        'R' => (pos, ( dir.1, -dir.0)),
        _ => panic!("Invalid command {}", cmd),
    };

    let p1 = |(pos, dir): State, cmd| match cmd {
        'N' => ((pos.0, pos.1 + 1), dir),
        'S' => ((pos.0, pos.1 - 1), dir),
        'E' => ((pos.0 + 1, pos.1), dir),
        'W' => ((pos.0 - 1, pos.1), dir),
        _ => common((pos, dir), cmd),
    };
    println!("Part 1: {}", run(&input, ((0, 0), (1, 0)), p1));

    let p2 = |(pos, way): State, cmd| match cmd {
        'N' => (pos, (way.0, way.1 + 1)),
        'S' => (pos, (way.0, way.1 - 1)),
        'E' => (pos, (way.0 + 1, way.1)),
        'W' => (pos, (way.0 - 1, way.1)),
        _ => common((pos, way), cmd),
    };
    println!("Part 2: {}", run(&input, ((0, 0), (10, 1)), p2));
}
