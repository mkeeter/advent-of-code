use std::collections::HashMap;
use std::io::BufRead;

#[derive(Copy, Clone, Default, Eq, PartialEq, Hash)]
struct Map {
    slab: [u8; 11],
    wells: [[u8; 2]; 4],
    locked: [[bool; 2]; 4],
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
impl Map {
    fn check(&self) {
        for (x, c) in self.slab.iter().enumerate() {
            if *c != 0 {
                assert!(!(x == 2 || x == 4 || x == 6 || x == 8));
            }
        }
    }
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

        let mut todo = Vec::new();

        // Moves from the slab into the well
        for (i, &c) in self.slab.iter().enumerate().filter(|(_, &c)| c != 0) {
            let target_well = (c - b'A') as usize;
            // Skip any moves that put us into an invalid target well
            if self.wells[target_well][1] != 0 && self.wells[target_well][1] != c {
                continue;
            }
            // Check whether the path is clear
            let target_well_x = target_well * 2 + 2;
            if (i.min(target_well_x)..=i.max(target_well_x)).any(|x| x != i && self.slab[x] != 0) {
                continue;
            }
            let well_bottom = if self.wells[target_well][1] == 0 {
                1
            } else {
                0
            };
            let mut next = *self;
            next.slab[i] = 0;
            next.wells[target_well][well_bottom] = c;
            next.check();
            next.locked[target_well][well_bottom] = true;

            let dx = i.max(target_well_x) - i.min(target_well_x);
            let dy = well_bottom + 1;

            todo.push((next, energy(c) * (dx + dy)));
        }

        // Moves from the wells into the slab
        for (w, well) in self.wells.iter().enumerate() {
            let y = if well[0] != 0 {
                0
            } else if well[1] != 0 {
                1
            } else {
                continue;
            };
            if self.locked[w][y] {
                continue;
            }
            let c = well[y];
            let well_x = w * 2 + 2;
            let mut x_targets = vec![];
            for x in (0..well_x).rev() {
                if self.slab[x] != 0 {
                    break;
                }
                x_targets.push(x);
            }
            for x in well_x..11 {
                if self.slab[x] != 0 {
                    break;
                }
                x_targets.push(x);
            }
            for x in x_targets.into_iter() {
                // Skip areas directly above wells
                if x == 2 || x == 4 || x == 6 || x == 8 {
                    continue;
                }
                let mut next = *self;
                next.wells[w][y] = 0;
                next.check();
                next.slab[x] = c;

                let dx = well_x.max(x) - well_x.min(x);
                let dy = y + 1;
                todo.push((next, energy(c) * (dx + dy)));
            }
        }
        let best = todo
            .into_iter()
            .filter_map(|(t, e)| t.explore(cache).map(|s| s + e))
            .min();
        cache.insert(*self, best);
        best
    }
}
impl std::fmt::Debug for Map {
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
        writeln!(f, "##")?;
        write!(f, "  #")?;
        for (w, well) in self.wells.iter().enumerate() {
            if well[1] == 0 {
                write!(f, ".#")?;
            } else if !self.locked[w][1] {
                write!(f, "{}#", (well[1] + b'a' - b'A') as char)?;
            } else {
                write!(f, "{}#", well[1] as char)?;
            }
        }
        }
        write!(f, "\n  #########")?;
        Ok(())
    }
}

fn main() {
    let mut map: Map = Map::default();
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
    println!("{:?}", map);
    let mut cache = HashMap::new();
    let res = map.explore(&mut cache);
    println!("{:?}", res);
    /*
    for m in res.1 {
        println!("{:?}", m);
    }
    */
}
