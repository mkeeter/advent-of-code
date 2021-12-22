use arrayvec::ArrayVec;
use parse_display::{Display, FromStr};
use std::io::BufRead;

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("{cmd} {region}")]
struct Command {
    cmd: String,
    region: Region,
}

#[derive(Copy, Clone, Display, FromStr, PartialEq, Debug)]
#[display("x={xmin}..{xmax},y={ymin}..{ymax},z={zmin}..{zmax}")]
struct Region {
    xmin: i64,
    xmax: i64,
    ymin: i64,
    ymax: i64,
    zmin: i64,
    zmax: i64,
}

impl Region {
    fn difference(&self, other: &Region) -> ArrayVec<Region, 26> {
        let mut out = ArrayVec::new();
        for x in 0..3 {
            for y in 0..3 {
                for z in 0..3 {
                    if x == 1 && y == 1 && z == 1 {
                        continue;
                    }
                    let (xmin, xmax) = match x {
                        0 => (self.xmin, other.xmin.min(self.xmax)),
                        1 => (other.xmin.max(self.xmin), other.xmax.min(self.xmax)),
                        2 => (other.xmax.max(self.xmin), self.xmax.min(self.xmax)),
                        _ => unreachable!(),
                    };
                    let (ymin, ymax) = match y {
                        0 => (self.ymin, other.ymin.min(self.ymax)),
                        1 => (other.ymin.max(self.ymin), other.ymax.min(self.ymax)),
                        2 => (other.ymax.max(self.ymin), self.ymax.min(self.ymax)),
                        _ => unreachable!(),
                    };
                    let (zmin, zmax) = match z {
                        0 => (self.zmin, other.zmin.min(self.zmax)),
                        1 => (other.zmin.max(self.zmin), other.zmax.min(self.zmax)),
                        2 => (other.zmax.max(self.zmin), self.zmax.min(self.zmax)),
                        _ => unreachable!(),
                    };
                    if xmax <= xmin || ymax <= ymin || zmax <= zmin {
                        continue;
                    }
                    out.push(Region {
                        xmin,
                        xmax,
                        ymin,
                        ymax,
                        zmin,
                        zmax,
                    });
                }
            }
        }
        loop {
            let mut compact: ArrayVec<Region, 26> = ArrayVec::new();
            // For each region, see if we can attach it to an existing
            // compacted region.
            for r in &out {
                let mut changed = false;
                for c in compact.iter_mut() {
                    if c.xmin == r.xmin && c.xmax == r.xmax && c.ymin == r.ymin && c.ymax == r.ymax
                    {
                        if r.zmin == c.zmax {
                            c.zmax = r.zmax;
                            changed = true;
                        } else if r.zmax == c.zmin {
                            c.zmin = r.zmin;
                            changed = true;
                        }
                    } else if c.ymin == r.ymin
                        && c.ymax == r.ymax
                        && c.zmin == r.zmin
                        && c.zmax == r.zmax
                    {
                        if r.xmin == c.xmax {
                            c.xmax = r.xmax;
                            changed = true;
                        } else if r.xmax == c.xmin {
                            c.xmin = r.xmin;
                            changed = true;
                        }
                    } else if c.zmin == r.zmin
                        && c.zmax == r.zmax
                        && c.xmin == r.xmin
                        && c.xmax == r.xmax
                    {
                        if r.ymin == c.ymax {
                            c.ymax = r.ymax;
                            changed = true;
                        } else if r.ymax == c.ymin {
                            c.ymin = r.ymin;
                            changed = true;
                        }
                    }
                    if changed {
                        break;
                    }
                }
                if !changed {
                    compact.push(*r);
                }
            }
            // Once we've stabilized, then return the compact vector
            if compact.len() == out.len() {
                return compact;
            }
            out = compact;
        }
    }
    fn far(&self) -> bool {
        self.xmax.min(self.ymax).min(self.zmax) < -50
            || self.xmin.max(self.ymin).max(self.zmin) > 50
    }
    fn volume(&self) -> i64 {
        (self.xmax - self.xmin) * (self.ymax - self.ymin) * (self.zmax - self.zmin)
    }
}

fn main() {
    let commands = std::io::stdin()
        .lock()
        .lines()
        .map(|line| {
            let mut c: Command = line.unwrap().parse().unwrap();
            c.region.xmax += 1;
            c.region.ymax += 1;
            c.region.zmax += 1;
            c
        })
        .collect::<Vec<Command>>();

    let mut lit: Vec<Region> = Vec::new();
    let mut next = Vec::new();
    for cmd in commands.iter().filter(|cmd| !cmd.region.far()) {
        next.clear();
        for r in &lit {
            next.extend(r.difference(&cmd.region).into_iter());
        }
        match cmd.cmd.as_str() {
            "on" => next.push(cmd.region),
            "off" => (),
            _ => panic!("Invalid command {}", cmd),
        }
        std::mem::swap(&mut lit, &mut next);
    }
    println!("Part 1: {}", lit.iter().map(|r| r.volume()).sum::<i64>());

    lit.clear();
    for cmd in commands.iter() {
        next.clear();
        for r in &lit {
            next.extend(r.difference(&cmd.region).into_iter());
        }
        match cmd.cmd.as_str() {
            "on" => next.push(cmd.region),
            "off" => (),
            _ => panic!("Invalid command {}", cmd),
        }
        std::mem::swap(&mut lit, &mut next);
    }
    println!("Part 2: {}", lit.iter().map(|r| r.volume()).sum::<i64>());
}
