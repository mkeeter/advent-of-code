use std::collections::{HashMap, HashSet};
pub fn solve(s: &str) -> (String, String) {
    let mut map = HashSet::new();
    let mut start = None;
    let height = s.lines().count() as i64;
    let width = s.lines().next().unwrap().len() as i64;
    for (y, line) in s.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            // The map contains valid empty squares
            match c {
                '#' => (),
                '.' => {
                    map.insert((x as i64, y as i64));
                }
                'S' => {
                    assert!(start.is_none());
                    start = Some((x as i64, y as i64));
                    map.insert((x as i64, y as i64));
                }
                c => panic!("invalid character {c}"),
            }
        }
    }
    let start = start.unwrap();
    let mut pos: HashSet<(i64, i64)> = [start].into_iter().collect();
    for _ in 0..64 {
        let mut next = HashSet::new();
        for p in pos.into_iter() {
            for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let n = (p.0 + dx, p.1 + dy);
                if map.contains(&n) {
                    next.insert(n);
                }
            }
        }
        pos = next;
    }
    let p1 = pos.len();

    ////////////////////////////////////////////////////////////////////////////

    // Each virtual map is even or odd.  Let's find how many steps it takes to
    // saturate a grid, so we can ignore them after that point.
    let mut pos: HashSet<(i64, i64)> = [start].into_iter().collect();
    let mut seen: HashSet<(i64, i64)> = pos.clone();
    let mut sat_even = 0;
    let mut sat_odd = 0;
    let mut corner_steps = None;
    for i in 0.. {
        let mut next = HashSet::new();
        for p in pos.into_iter() {
            if p.0 == width - 1 && p.1 == height - 1 && corner_steps.is_none() {
                corner_steps = Some(i);
            }

            if i % 2 == 0 {
                sat_even += 1;
            } else {
                sat_odd += 1;
            }
            for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let n = (p.0 + dx, p.1 + dy);
                if map.contains(&n) && seen.insert(n) {
                    next.insert(n);
                }
            }
        }
        pos = next;
        if pos.is_empty() {
            break;
        }
    }
    let sat_count = seen.len();
    let corner_steps = corner_steps.unwrap() + 2; // to get to the next grid

    const MAX_STEPS: i64 = 50;

    // Find out how many steps it takes to saturate the map starting from that
    // corner
    let mut even = 0;
    let mut odd = 0;
    let mut sat_steps_even = None;
    let mut sat_steps_odd = None;
    let mut pos: HashSet<(i64, i64)> = [(0, 0)].into_iter().collect();
    let mut seen: HashSet<(i64, i64)> = pos.clone();
    for i in 0.. {
        let mut next = HashSet::new();
        for p in pos.into_iter() {
            if i % 2 == 0 {
                even += 1;
                if even == sat_even {
                    sat_steps_even = Some(i);
                }
            } else {
                odd += 1;
                if odd == sat_odd {
                    sat_steps_odd = Some(i);
                }
            }
            for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let n = (p.0 + dx, p.1 + dy);
                if map.contains(&n)
                    && n.0 >= 0
                    && n.0 < width
                    && n.1 >= 0
                    && n.1 < height
                    && seen.insert(n)
                {
                    next.insert(n);
                }
            }
        }
        pos = next;
        if pos.is_empty() {
            break;
        }
    }
    let sat_steps_even = sat_steps_even.unwrap();
    let sat_steps_odd = sat_steps_odd.unwrap();

    struct Quadtree {
        scale: u64,
        node: Node,
    }
    enum Node {
        Empty,
        Split(Box<[Node; 4]>),
    }

    assert_eq!(width, height);
    let max_sat_steps = sat_steps_even.max(sat_steps_odd);

    // solve for
    // (width + height) * (scale - 1) < MAX_STEPS - max_sat_steps - corner_steps
    let moooo = (MAX_STEPS - max_sat_steps - corner_steps) / width;
    println!("even sat steps: {sat_steps_even}");
    println!("odd sat steps: {sat_steps_odd}");
    println!("max sat steps: {max_sat_steps}");
    println!("steps to corner: {corner_steps}");
    println!("steps to edge: {moooo}");

    panic!("oh no");

    let mut pos: HashSet<((i64, i64), (i64, i64))> =
        [(start, (0, 0))].into_iter().collect();
    let mut seen: HashMap<(i64, i64), HashSet<(i64, i64)>> = HashMap::new();
    let mut sat: HashSet<(i64, i64)> = HashSet::new();
    let mut radius = -1;
    let mut circumferences = HashMap::new();

    for i in 0..5000 {
        let mut next = HashSet::new();

        if i % 100 == 0 {
            println!(
                "{i}: got {} pos, {} sat, {} active, {radius}",
                pos.len(),
                sat.len(),
                seen.len()
            );
        }
        for ((px, py), (gx, gy)) in pos.into_iter() {
            assert!(px >= 0);
            assert!(py >= 0);
            assert!(px < width);
            assert!(py < height);

            // Skip grid cells which are entirely out of our radius
            if gx.abs().max(gy.abs()) <= radius {
                continue;
            }

            // Skip full squares
            if !map.contains(&(px, py)) {
                continue;
            }

            // Skip saturated grids
            if sat.contains(&(gx, gy)) {
                continue;
            }

            // Skip points that we've already seen in unsaturated grids
            let sub = seen.entry((gx, gy)).or_default();
            if !sub.insert((px, py)) {
                continue;
            }

            // Saturate the grid, removing it from the dense list
            if sub.len() == sat_count {
                // This is no longer in the dense grid map
                seen.remove(&(gx, gy)).unwrap();
                // It's now in the saturated grid map
                let prev = sat.insert((gx, gy));
                assert!(prev); // and must be newly inserted

                let this_radius = gx.abs().max(gy.abs());
                let circ: &mut i64 =
                    circumferences.entry(this_radius).or_default();
                *circ += 1;
                let target_circ = if this_radius == 0 {
                    1
                } else {
                    let d = this_radius * 2 + 1;
                    d.pow(2) - (d - 2).pow(2)
                };
                if *circ == target_circ {
                    assert_eq!(this_radius, radius + 1);
                    radius = this_radius;
                    let p = sat.len();
                    sat.retain(|(gx, gy)| gx.abs().max(gy.abs()) > radius);
                    println!("{p} -> {} ({})", sat.len(), p - sat.len());
                    if p - sat.len() == 16 {
                        println!("{sat:?}");
                    }
                }
            }

            for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let mut nx = px + dx;
                let mut ny = py + dy;
                let gx = gx
                    + if nx < 0 {
                        nx += width;
                        -1
                    } else if nx >= width {
                        nx -= width;
                        1
                    } else {
                        0
                    };
                let gy = gy
                    + if ny < 0 {
                        ny += height;
                        -1
                    } else if ny >= height {
                        ny -= height;
                        1
                    } else {
                        0
                    };

                next.insert(((nx, ny), (gx, gy)));
            }
        }
        pos = next;
    }
    (p1.to_string(), "unimplemented".to_owned())
}
