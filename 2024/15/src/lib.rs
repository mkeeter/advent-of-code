use std::collections::HashMap;

pub fn solve(s: &str) -> (u64, u64) {
    let mut grid = HashMap::new();
    let mut start = None;
    let mut map = true;
    let mut commands = vec![];
    for (y, line) in s.lines().enumerate() {
        if line.is_empty() {
            map = false;
        }
        if map {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '.' => (),
                    '@' => start = Some((x as i64, y as i64)),
                    '#' | 'O' => {
                        grid.insert((x as i64, y as i64), c);
                    }
                    _ => panic!("invalid grid character '{c}'"),
                }
            }
        } else {
            for c in line.chars() {
                commands.push(match c {
                    'v' => (0, 1),
                    '^' => (0, -1),
                    '>' => (1, 0),
                    '<' => (-1, 0),
                    _ => panic!("invalid direction '{c}'"),
                })
            }
        }
    }

    let (mut x, mut y) = start.expect("could not find start");
    'outer: for &(dx, dy) in &commands {
        let nx = x + dx;
        let ny = y + dy;
        match grid.get(&(nx, ny)) {
            None => (),
            Some('#') => continue, // bonk
            Some('O') => {
                let (mut gx, mut gy) = (nx, ny);
                let (gx, gy) = loop {
                    gx += dx;
                    gy += dy;
                    match grid.get(&(gx, gy)) {
                        None => break (gx, gy),
                        Some('#') => continue 'outer,
                        Some('O') => (),
                        c => panic!("invalid grid value {c:?}"),
                    }
                };
                let v = grid.remove(&(nx, ny)).unwrap();
                assert_eq!(v, 'O');
                let prev = grid.insert((gx, gy), v);
                assert!(prev.is_none());
            }
            c => panic!("invalid grid value {c:?}"),
        }
        (x, y) = (nx, ny);
    }

    let score = |grid: &HashMap<(i64, i64), char>, c| {
        grid.iter()
            .filter(|(_p, v)| **v == c)
            .map(|((x, y), _v)| (y * 100 + x) as u64)
            .sum()
    };
    let part1 = score(&grid, 'O');

    // reparse into a wiiiiiiiiiiide grid
    let mut grid = HashMap::new();
    let mut start = None;
    for (y, line) in s.lines().enumerate() {
        if line.is_empty() {
            break; // commands don't change
        }
        let y = y as i64;
        for (x, c) in line.chars().enumerate() {
            let x = x as i64;
            match c {
                '.' => (),
                '@' => start = Some((x * 2, y)),
                '#' => {
                    grid.insert((x * 2, y), '#');
                    grid.insert((x * 2 + 1, y), '#');
                }
                'O' => {
                    grid.insert((x * 2, y), '[');
                    grid.insert((x * 2 + 1, y), ']');
                }
                _ => panic!("invalid grid character '{c}'"),
            }
        }
    }
    let (mut x, mut y) = start.expect("could not find start");
    for &(dx, dy) in &commands {
        assert!(!grid.contains_key(&(x, y)));

        let nx = x + dx;
        let ny = y + dy;
        match grid.get(&(nx, ny)) {
            None => (),
            Some('#') => continue, // bonk
            Some('[' | ']') if dy == 0 => {
                if !push_hbox((nx, ny), dx, &mut grid) {
                    continue;
                }
            }
            Some('[') => {
                let mut temp = grid.clone();
                if !push_vbox((nx, ny), dy, &mut temp) {
                    continue;
                }
                grid = temp;
            }
            Some(']') => {
                let mut temp = grid.clone();
                if !push_vbox((nx - 1, ny), dy, &mut temp) {
                    continue;
                }
                grid = temp;
            }
            c => panic!("invalid grid value {c:?}"),
        }
        assert!(!grid.contains_key(&(nx, ny)));
        (x, y) = (nx, ny);
    }
    let part2 = score(&grid, '[');

    (part1, part2)
}

fn push_hbox(
    corner: (i64, i64),
    dx: i64,
    grid: &mut HashMap<(i64, i64), char>,
) -> bool {
    // Horizontal pushing is easy!
    let (mut gx, gy) = corner;
    loop {
        gx += dx;
        match grid.get(&(gx, gy)) {
            None => {
                break;
            }
            Some('#') => return false, // bonk
            Some('[' | ']') => (),
            Some(c) => panic!("invalid grid value {c:?}"),
        }
    }
    // Slide all of the boxes along
    while gx != corner.0 {
        let v = grid.remove(&(gx - dx, gy)).unwrap();
        assert!(v == '[' || v == ']', "bad hbox '{v}'");
        let prev = grid.insert((gx, gy), v);
        assert!(prev.is_none());
        gx -= dx;
    }
    true
}

fn push_vbox(
    corner: (i64, i64),
    dy: i64,
    grid: &mut HashMap<(i64, i64), char>,
) -> bool {
    assert_eq!(grid[&corner], '[');
    assert_eq!(grid[&(corner.0 + 1, corner.1)], ']');

    for offset in [0, 1] {
        let (nx, ny) = (corner.0 + offset, corner.1 + dy);
        match grid.get(&(nx, ny)) {
            None => (),
            Some('#') => return false, // bonk
            Some('[') => {
                if !push_vbox((nx, ny), dy, grid) {
                    return false;
                }
            }
            Some(']') => {
                if !push_vbox((nx - 1, ny), dy, grid) {
                    return false;
                }
            }
            Some(c) => panic!("invalid grid value '{c}'"),
        }
    }
    let v = grid.remove(&corner).unwrap();
    assert_eq!(v, '[');
    let prev = grid.insert((corner.0, corner.1 + dy), v);
    assert!(prev.is_none());

    let v = grid.remove(&(corner.0 + 1, corner.1)).unwrap();
    assert_eq!(v, ']');
    let prev = grid.insert((corner.0 + 1, corner.1 + dy), v);
    assert!(prev.is_none());

    true
}

#[allow(unused)]
fn print_grid(pos: (i64, i64), grid: &HashMap<(i64, i64), char>) {
    let width = grid.keys().map(|(x, _)| x).max().cloned().unwrap_or(0);
    let height = grid.keys().map(|(_, y)| y).max().cloned().unwrap_or(0);
    for y in 0..=height {
        for x in 0..=width {
            let p = (x, y);
            if p == pos {
                assert!(!grid.contains_key(&p));
                print!("@");
            } else {
                print!("{}", grid.get(&p).unwrap_or(&'.'));
            }
        }
        println!();
    }
}

#[allow(unused)]
fn check_grid(
    pos: (i64, i64),
    grid: &HashMap<(i64, i64), char>,
) -> (usize, usize) {
    let width = grid.keys().map(|(x, _)| x).max().cloned().unwrap_or(0);
    let height = grid.keys().map(|(_, y)| y).max().cloned().unwrap_or(0);
    let mut box_count = 0;
    let mut wall_count = 0;
    for y in 0..=height {
        for x in 0..=width {
            let p = (x, y);
            if p == pos {
                assert!(!grid.contains_key(&p));
            } else {
                match grid.get(&p) {
                    None => (),
                    Some('#') => wall_count += 1,
                    Some('[') => {
                        box_count += 1;
                        assert_eq!(grid.get(&(x + 1, y)), Some(&']'))
                    }
                    Some(']') => assert_eq!(grid.get(&(x - 1, y)), Some(&'[')),
                    Some(c) => panic!("invalid grid cell '{c}'"),
                }
            }
        }
    }
    (box_count, wall_count)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        const SMALL: &str = indoc::indoc! {"
            ########
            #..O.O.#
            ##@.O..#
            #...O..#
            #.#.O..#
            #...O..#
            #......#
            ########

            <^^>>>vv<v>>v<<
        "};
        assert_eq!(solve(SMALL).0, 2028);

        println!("\n------------------\n");
        const LARGE: &str = indoc::indoc! {"
            ##########
            #..O..O.O#
            #......O.#
            #.OO..O.O#
            #..O@..O.#
            #O#..O...#
            #O..O..O.#
            #.OO.O.OO#
            #....O...#
            ##########

            <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
            vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
            ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
            <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
            ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
            ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
            >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
            <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
            ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
            v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
        "};
        assert_eq!(solve(LARGE), (10092, 9021));
    }
}
