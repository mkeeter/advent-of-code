use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use util::{Dir, Grid};

#[derive(Copy, Clone, Debug)]
struct Pos {
    x: i64,
    y: i64,
    t: usize,
    cheat: Option<Cheat>,
}

#[derive(Copy, Clone, Debug)]
struct Cheat {
    t: usize,
    x: i64,
    y: i64,
}

fn recurse(
    g: &Grid,
    p: Pos,
    end: (i64, i64),
    max_time: usize,
    seen: &mut HashSet<(i64, i64)>,
    found: &mut HashMap<(i64, i64), usize>,
) {
    if p.t >= max_time {
        return;
    } else if (p.x, p.y) == end {
        if let Some(c) = p.cheat {
            let e = found.entry((c.x, c.y)).or_insert(p.t);
            *e = (*e).min(p.t);
        }
        return;
    } else if !seen.insert((p.x, p.y)) {
        return;
    }

    for d in Dir::iter() {
        let nx = p.x + d.x();
        let ny = p.y + d.y();
        let blocked = matches!(g.get(nx, ny), None | Some(b'#'));
        if !blocked {
            recurse(
                g,
                Pos {
                    t: p.t + 1,
                    x: nx,
                    y: ny,
                    cheat: p.cheat,
                },
                end,
                max_time,
                seen,
                found,
            );
        } else if let Some(c) = p.cheat {
            if c.t == p.t {
                recurse(
                    g,
                    Pos {
                        t: p.t + 1,
                        x: nx,
                        y: ny,
                        cheat: Some(c),
                    },
                    end,
                    max_time,
                    seen,
                    found,
                );
            }
        } else {
            recurse(
                g,
                Pos {
                    t: p.t + 1,
                    x: nx,
                    y: ny,
                    cheat: Some(Cheat {
                        t: p.t,
                        x: p.x,
                        y: p.y,
                    }),
                },
                end,
                max_time,
                seen,
                found,
            );
        }
    }
    seen.remove(&(p.x, p.y));
}

pub fn solve(s: &str) -> (u64, u64) {
    let g = Grid::new(s);
    let mut start = None;
    let mut end = None;
    for y in 0..g.height() {
        for x in 0..g.width() {
            match g[(x, y)] {
                b'S' => start = Some((x, y)),
                b'E' => end = Some((x, y)),
                b'#' | b'.' => (),
                c => panic!("invalid grid character '{c}'"),
            }
        }
    }
    let start = start.unwrap();
    let end = end.unwrap();

    let mut seen = HashSet::new();
    let mut todo = VecDeque::new();
    todo.push_back((0, start.0, start.1));
    let mut shortest = None;
    while let Some((t, x, y)) = todo.pop_front() {
        if (x, y) == end {
            shortest = Some(t);
            break;
        }
        if !seen.insert((x, y)) {
            continue;
        }
        for d in Dir::iter() {
            let x = x + d.x();
            let y = y + d.y();
            if g[(x, y)] != b'#' {
                todo.push_back((t + 1, x, y));
            }
        }
    }
    let shortest = shortest.unwrap();
    println!("shortest path: {shortest}");

    let mut todo = VecDeque::new();
    let mut seen = HashSet::new();
    todo.push_back(Pos {
        x: start.0,
        y: start.1,
        t: 0,
        cheat: None,
    });
    let mut found = HashMap::new();
    while let Some(p) = todo.pop_front() {
        if p.t >= shortest {
            continue;
        } else if (p.x, p.y) == end {
            if let Some(c) = p.cheat {
                let e = found.entry((c.x, c.y)).or_insert(p.t);
                *e = (*e).min(p.t);
            }
            continue;
        } else if !seen.insert((p.x, p.y, p.cheat.map(|c| (c.x, c.y)))) {
            continue;
        }

        for d in Dir::iter() {
            let nx = p.x + d.x();
            let ny = p.y + d.y();
            let blocked = matches!(g.get(nx, ny), None | Some(b'#'));
            if !blocked {
                todo.push_back(Pos {
                    t: p.t + 1,
                    x: nx,
                    y: ny,
                    cheat: p.cheat,
                });
            } else if let Some(c) = p.cheat {
                if c.t == p.t {
                    println!("!!");
                    todo.push_back(Pos {
                        t: p.t + 1,
                        x: nx,
                        y: ny,
                        cheat: Some(c),
                    });
                }
            } else {
                todo.push_back(Pos {
                    t: p.t + 1,
                    x: nx,
                    y: ny,
                    cheat: Some(Cheat {
                        t: p.t,
                        x: p.x,
                        y: p.y,
                    }),
                });
            }
        }
    }
    let mut skip_count: BTreeMap<usize, usize> = BTreeMap::new();
    for t in found.values() {
        *skip_count.entry(shortest - t).or_default() += 1;
    }
    for (n, count) in &skip_count {
        println!("{n} => {count}");
    }
    println!("{}", found.len());

    (0, 0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        const EXAMPLE: &str = indoc::indoc! {"
            ###############
            #...#...#.....#
            #.#.#.#.#.###.#
            #S#...#.#.#...#
            #######.#.#.###
            #######.#.#...#
            #######.#.###.#
            ###..E#...#...#
            ###.#######.###
            #...###...#...#
            #.#####.#.###.#
            #.#...#.#.#...#
            #.#.#.#.#.#.###
            #...#...#...###
            ###############
        "};
        assert_eq!(solve(EXAMPLE), (0, 0));

        /*
        const SIMPLE: &str = indoc::indoc! {"
            #######
            #...#E#
            #S#...#
            #######
        "};
        assert_eq!(solve(SIMPLE), (0, 0));
        */
    }
}
