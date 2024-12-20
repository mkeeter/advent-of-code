use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use util::{Dir, Grid};

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Pos {
    t: usize,
    x: i64,
    y: i64,
    cheat: Option<Cheat>,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Cheat {
    start: (i64, i64),
    end: (i64, i64),
}

struct Recurse<'a> {
    g: Grid<'a>,
    end: (i64, i64),
    max_time: usize,
    seen: HashSet<(i64, i64)>,
    found: HashMap<Cheat, usize>,
}

impl<'a> Recurse<'a> {
    fn new(g: Grid<'a>, max_time: usize, end: (i64, i64)) -> Self {
        Recurse {
            g,
            max_time,
            end,
            seen: HashSet::new(),
            found: HashMap::new(),
        }
    }

    fn recurse(&mut self, p: Pos) {
        if p.t >= self.max_time {
            return;
        } else if (p.x, p.y) == self.end {
            if let Some(c) = p.cheat {
                let e = self.found.entry(c).or_insert(p.t);
                *e = (*e).min(p.t);
            }
            return;
        } else if !self.seen.insert((p.x, p.y)) {
            return;
        }

        for d in Dir::iter() {
            let x = p.x + d.x();
            let y = p.y + d.y();
            if !self.blocked(x, y) {
                self.recurse(Pos {
                    t: p.t + 1,
                    x,
                    y,
                    cheat: p.cheat,
                });
            } else if p.cheat.is_none() {
                for d in Dir::iter() {
                    let x = x + d.x();
                    let y = y + d.y();
                    if !self.blocked(x, y) {
                        self.recurse(Pos {
                            t: p.t + 2,
                            x,
                            y,
                            cheat: Some(Cheat {
                                start: (p.x, p.y),
                                end: (x, y),
                            }),
                        });
                    }
                }
            }
        }
        self.seen.remove(&(p.x, p.y));
    }

    fn blocked(&self, x: i64, y: i64) -> bool {
        matches!(self.g.get(x, y), None | Some(b'#'))
    }
}

pub fn solve(s: &str) -> (usize, u64) {
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

    let mut r = Recurse::new(g, shortest, end);

    let mut todo = BTreeSet::new();
    let mut seen = HashSet::new();
    todo.insert(Pos {
        t: 0,
        x: start.0,
        y: start.1,
        cheat: None,
    });
    let mut found = HashMap::new();
    while let Some(p) = todo.pop_first() {
        if p.t >= shortest {
            continue;
        } else if (p.x, p.y) == end {
            if let Some(c) = p.cheat {
                println!(
                    "found path saving {} with cheat at {:?},{:?}",
                    shortest - p.t,
                    c.start,
                    c.end,
                );
                let e = found.entry(c).or_insert(p.t);
                *e = (*e).min(p.t);
            }
            continue;
        } else if !seen.insert((p.x, p.y, p.cheat)) {
            continue;
        }

        for d in Dir::iter() {
            let nx = p.x + d.x();
            let ny = p.y + d.y();
            let blocked = matches!(g.get(nx, ny), None | Some(b'#'));
            if !blocked {
                todo.insert(Pos {
                    t: p.t + 1,
                    x: nx,
                    y: ny,
                    cheat: p.cheat,
                });
            } else if p.cheat.is_none() {
                for d in Dir::iter() {
                    let cx = nx + d.x();
                    let cy = ny + d.y();
                    let blocked = matches!(g.get(cx, cy), None | Some(b'#'));
                    if !blocked {
                        todo.insert(Pos {
                            t: p.t + 2,
                            x: cx,
                            y: cy,
                            cheat: Some(Cheat {
                                start: (p.x, p.y),
                                end: (cx, cy),
                            }),
                        });
                    }
                }
            }
        }
    }
    let mut skip_count: BTreeMap<usize, usize> = BTreeMap::new();
    for t in found.values() {
        *skip_count.entry(shortest - t).or_default() += 1;
    }
    let mut out = 0;
    for (saved, count) in &skip_count {
        println!("{saved} => {count}");
        if *saved >= 100 {
            out += count;
        }
    }

    (out, 0)
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

        /*
        const EXAMPLE: &str = indoc::indoc! {"
            ###############
            #...#...#.....#
            #.#.#.#.#.###.#
            #S#...#.#.#...#
            #######.#.#.###
            #######.#.#...#
            #######.#.###.#
            ###...#...#E..#
            ###############
        "};
        assert_eq!(solve(EXAMPLE), (0, 0));
        */
    }
}
