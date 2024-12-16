use std::collections::HashMap;

enum Cell {
    Box,
    Wall,
}

#[derive(Clone)]
enum WideCell {
    LeftBox,
    RightBox,
    Wall,
}

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
            let y = y as i64;
            for (x, c) in line.chars().enumerate() {
                let x = x as i64;
                match c {
                    '.' => (),
                    '@' => start = Some((x, y)),
                    '#' => {
                        grid.insert((x, y), Cell::Wall);
                    }
                    'O' => {
                        grid.insert((x, y), Cell::Box);
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
            Some(Cell::Wall) => continue, // bonk
            Some(Cell::Box) => {
                let (mut gx, mut gy) = (nx, ny);
                let (gx, gy) = loop {
                    gx += dx;
                    gy += dy;
                    match grid.get(&(gx, gy)) {
                        None => break (gx, gy),
                        Some(Cell::Wall) => continue 'outer,
                        Some(Cell::Box) => (),
                    }
                };
                let v = grid.remove(&(nx, ny)).unwrap();
                assert!(matches!(v, Cell::Box));
                let prev = grid.insert((gx, gy), v);
                assert!(prev.is_none());
            }
        }
        (x, y) = (nx, ny);
    }

    let part1 = grid
        .iter()
        .filter(|(_p, v)| matches!(*v, Cell::Box))
        .map(|((x, y), _v)| (y * 100 + x) as u64)
        .sum();

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
                    grid.insert((x * 2, y), WideCell::Wall);
                    grid.insert((x * 2 + 1, y), WideCell::Wall);
                }
                'O' => {
                    grid.insert((x * 2, y), WideCell::LeftBox);
                    grid.insert((x * 2 + 1, y), WideCell::RightBox);
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
            Some(WideCell::Wall) => continue, // bonk
            Some(WideCell::LeftBox | WideCell::RightBox) if dy == 0 => {
                if !push_hbox((nx, ny), dx, &mut grid) {
                    continue;
                }
            }
            Some(WideCell::LeftBox) => {
                let mut temp = grid.clone();
                if !push_vbox((nx, ny), dy, &mut temp) {
                    continue;
                }
                grid = temp;
            }
            Some(WideCell::RightBox) => {
                let mut temp = grid.clone();
                if !push_vbox((nx - 1, ny), dy, &mut temp) {
                    continue;
                }
                grid = temp;
            }
        }
        assert!(!grid.contains_key(&(nx, ny)));
        (x, y) = (nx, ny);
    }
    let part2 = grid
        .iter()
        .filter(|(_p, v)| matches!(**v, WideCell::LeftBox))
        .map(|((x, y), _v)| (y * 100 + x) as u64)
        .sum();

    (part1, part2)
}

fn push_hbox(
    corner: (i64, i64),
    dx: i64,
    grid: &mut HashMap<(i64, i64), WideCell>,
) -> bool {
    // Horizontal pushing is easy!
    let (mut gx, gy) = corner;
    loop {
        gx += dx;
        match grid.get(&(gx, gy)) {
            None => {
                break;
            }
            Some(WideCell::Wall) => return false, // bonk
            Some(WideCell::LeftBox | WideCell::RightBox) => (),
        }
    }
    // Slide all of the boxes along
    while gx != corner.0 {
        let v = grid.remove(&(gx - dx, gy)).unwrap();
        assert!(matches!(v, WideCell::LeftBox | WideCell::RightBox));
        let prev = grid.insert((gx, gy), v);
        assert!(prev.is_none());
        gx -= dx;
    }
    true
}

fn push_vbox(
    corner: (i64, i64),
    dy: i64,
    grid: &mut HashMap<(i64, i64), WideCell>,
) -> bool {
    assert!(matches!(grid[&corner], WideCell::LeftBox));
    assert!(matches!(
        grid[&(corner.0 + 1, corner.1)],
        WideCell::RightBox
    ));

    for offset in [0, 1] {
        let (nx, ny) = (corner.0 + offset, corner.1 + dy);
        match grid.get(&(nx, ny)) {
            None => (),
            Some(WideCell::Wall) => return false, // bonk
            Some(WideCell::LeftBox) => {
                if !push_vbox((nx, ny), dy, grid) {
                    return false;
                }
            }
            Some(WideCell::RightBox) => {
                if !push_vbox((nx - 1, ny), dy, grid) {
                    return false;
                }
            }
        }
    }
    let v = grid.remove(&corner).unwrap();
    assert!(matches!(v, WideCell::LeftBox));
    let prev = grid.insert((corner.0, corner.1 + dy), v);
    assert!(prev.is_none());

    let v = grid.remove(&(corner.0 + 1, corner.1)).unwrap();
    assert!(matches!(v, WideCell::RightBox));
    let prev = grid.insert((corner.0 + 1, corner.1 + dy), v);
    assert!(prev.is_none());

    true
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
