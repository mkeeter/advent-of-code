use std::collections::HashMap;
use std::io::BufRead;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Map<const DEPTH: usize> {
    slab: [u8; 11],
    wells: [[u8; DEPTH]; 4],
    locked: [[bool; DEPTH]; 4],
}

fn energy(b: u8) -> usize {
    match b {
        b'A' => 1,
        b'B' => 10,
        b'C' => 100,
        b'D' => 1000,
        _ => panic!("Invalid ambipod '{}'", b),
    }
}

impl<const DEPTH: usize> Default for Map<DEPTH> {
    fn default() -> Self {
        Map {
            slab: [0; 11],
            wells: [[0; DEPTH]; 4],
            locked: [[false; DEPTH]; 4],
        }
    }
}

impl<const DEPTH: usize> Map<DEPTH> {
    fn explore(&self, cache: &mut HashMap<Self, Option<usize>>) -> Option<usize> {
        if let Some(s) = cache.get(self) {
            return *s;
        }

        // Are we done?
        if self
            .wells
            .iter()
            .enumerate()
            .all(|(i, w)| w.iter().all(|c| *c == b'A' + i as u8))
        {
            return Some(0);
        }

        let mut best = None;
        let mut recurse = |m: Map<DEPTH>, energy: usize| {
            if let Some(score) = m.explore(cache) {
                let score = score + energy;
                if best.is_none() || score < best.unwrap() {
                    best = Some(score);
                }
            }
        };

        // Moves from the slab into the well
        for (i, &c) in self.slab.iter().enumerate().filter(|(_, &c)| c != 0) {
            let w = (c - b'A') as usize;

            // Skip any wells which have non-final characters
            if self.wells[w].iter().any(|&d| d != 0 && d != c) {
                continue;
            }

            // Check whether the path is clear
            let well_x = w * 2 + 2;
            if (i.min(well_x)..=i.max(well_x)).any(|x| x != i && self.slab[x] != 0) {
                continue;
            }

            let y =
                if let Some(w) = (0..DEPTH).rev().find(|y| self.wells[w][*y] == 0) {
                    w
                } else {
                    continue;
                };

            let mut next = *self;
            next.slab[i] = 0;
            next.wells[w][y] = c;
            next.locked[w][y] = true;

            let dx = i.max(well_x) - i.min(well_x);
            let dy = y + 1;

            recurse(next, energy(c) * (dx + dy));
        }

        // Moves from the wells into the slab
        for (w, well) in self.wells.iter().enumerate() {
            let y = if let Some(w) = (0..DEPTH).find(|y| well[*y] != 0) {
                w
            } else {
                continue;
            };
            if self.locked[w][y] {
                continue;
            }
            let well_x = w * 2 + 2;
            let c = well[y];
            let mut enqueue = |x| {
                // Skip areas directly above wells
                if x != 2 && x != 4 && x != 6 && x != 8 {
                    let mut next = *self;
                    next.wells[w][y] = 0;
                    next.slab[x] = c;

                    let dx = well_x.max(x) - well_x.min(x);
                    let dy = y + 1;
                    recurse(next, energy(c) * (dx + dy));
                }
            };
            for x in (0..well_x).rev() {
                if self.slab[x] != 0 {
                    break;
                }
                enqueue(x);
            }
            for x in well_x..self.slab.len() {
                if self.slab[x] != 0 {
                    break;
                }
                enqueue(x);
            }
        }
        cache.insert(*self, best);
        best
    }
}

impl<const DEPTH: usize> std::fmt::Debug for Map<DEPTH> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(f, "#############")?;
        write!(f, "#")?;
        for c in self.slab.iter() {
            if *c == 0 {
                write!(f, ".")?;
            } else {
                write!(f, "{}", *c as char)?;
            }
        }
        write!(f, "#\n###")?;
        for (w, well) in self.wells.iter().enumerate() {
            if well[0] == 0 {
                write!(f, ".#")?;
            } else if !self.locked[w][0] {
                write!(f, "{}#", (well[0] + b'a' - b'A') as char)?;
            } else {
                write!(f, "{}#", well[0] as char)?;
            }
        }
        writeln!(f, "##")?;
        for y in 1..DEPTH {
            write!(f, "  #")?;
            for (w, well) in self.wells.iter().enumerate() {
                if well[y] == 0 {
                    write!(f, ".#")?;
                } else if !self.locked[w][y] {
                    write!(f, "{}#", (well[y] + b'a' - b'A') as char)?;
                } else {
                    write!(f, "{}#", well[y] as char)?;
                }
            }
            writeln!(f)?;
        }
        write!(f, "  #########")?;
        Ok(())
    }
}

fn main() {
    let mut map: Map<2> = Map::default();
    for (y, line) in std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .enumerate()
    {
        for (x, c) in line.bytes().enumerate() {
            if c.is_ascii_uppercase() {
                let depth = y - 2;
                let well = (x - 3) / 2;
                map.wells[well][depth] = c;
            }
        }
    }
    let mut cache = HashMap::new();
    println!("Part 1: {}", map.explore(&mut cache).unwrap());

    let mut map2: Map<4> = Map::default();
    for (w, well) in map.wells.iter().enumerate() {
        map2.wells[w][0] = well[0];
        map2.wells[w][1] = "DCBA".as_bytes()[w];
        map2.wells[w][2] = "DBAC".as_bytes()[w];
        map2.wells[w][3] = well[1];
    }
    let mut cache = HashMap::new();
    println!("Part 2: {}", map2.explore(&mut cache).unwrap());
}
