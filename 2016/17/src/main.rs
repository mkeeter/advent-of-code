use std::collections::VecDeque;
use std::io::Read;

fn main() {
    let mut salt = String::new();
    std::io::stdin().read_to_string(&mut salt).unwrap();
    let salt = salt.trim();

    let directions = [('U', 0, -1), ('D', 0, 1), ('L', -1, 0), ('R', 1, 0)];

    let mut todo = VecDeque::new();
    todo.push_back((0, 0, "".to_string()));
    let mut longest = 0;
    while let Some((x, y, path)) = todo.pop_front() {
        if x == 3 && y == 3 {
            if longest == 0 {
                println!("Part 1: {}", path);
            }
            if path.len() > longest {
                longest = path.len();
            }
            continue;
        }
        format!("{:x}", md5::compute(salt.to_string() + &path))
            .chars()
            .zip(directions.iter())
            // Check whether the doors are open
            .filter(|&(c, _d)| c >= 'b' && c <= 'f')
            // Walk in the given direction
            .map(|(_c, (dir, dx, dy))| (dir, x + dx, y + dy))
            // Confirm that we're still on the map
            .filter(|&(_dir, x, y)| x >= 0 && y >= 0 && x < 4 && y < 4)
            // Accumulate in our vecdeque
            .for_each(|(dir, x, y)| todo.push_back(
                    (x, y, format!("{}{}", path, dir))))
    }
    println!("Part 2: {}", longest);
}
