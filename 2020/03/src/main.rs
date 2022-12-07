use std::io::BufRead;

fn main() {
    let lines: Vec<String> = std::io::stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .collect();
    let width = lines[0].chars().count();
    let height = lines.len();
    assert!(lines.iter().all(|line| line.chars().count() == width));

    let trees: Vec<Vec<bool>> = lines
        .iter()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect();

    let check = |dx: usize, dy: usize| {
        (0..((height + dy - 1) / dy))
            .filter(|i| trees[dy * i][(dx * i) % width])
            .count()
    };

    println!("Part 1: {}", check(3, 1));

    let p2: usize = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|&(x, y)| check(x, y))
        .product();
    println!("Part 2: {}", p2);
}
