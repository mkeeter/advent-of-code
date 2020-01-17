use std::str::FromStr;
use std::collections::HashMap;
use std::io::BufRead;

fn main() {
    let mut nodes = HashMap::new();
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        let i = line.replace('T', "")
            .replace("-x", " ")
            .replace("-y", " ")
            .split(' ')
            .filter_map(|w| i32::from_str(w).ok())
            .collect::<Vec<i32>>();
        if i.len() == 5 {
            // (x, y) -> (size, used, avail)
            nodes.insert((i[0], i[1]), (i[2], i[3], i[4]));
        }
    }

    let (width, _height) = {
        let (w, h) = nodes.keys().max().unwrap();
        (w + 1, h + 1)
    };

    let mut viable = 0;
    for (i, a) in nodes.iter() {
        for (j, b) in nodes.iter() {
            // Skip if both are the same node
            if i == j {
                continue;
            }
            // Compare (size, used, avail)
            if a.1 != 0 && a.1 <= b.2 {
                viable += 1;
            }
        }
    }
    println!("Part 1: {}", viable);

    // We assume that our input has the following structure:
    //
    //  0 o o #
    //  o o o o
    //  o X X X
    //  o . o o
    //
    //  with the following key:
    //      X   tile with too much data to ever move
    //      o   tile with data that can shift into non-X neighbors
    //      .   a single hole (completely empty node)
    //      #   our target data
    //      0   the node to which we should bring the data
    let hole = nodes.iter()
        .find(|(_pos, (_size, used, _avail))| *used == 0)
        .unwrap().0;

    let steps =
        // First, walk the hole left to (0, hole.1),
        // since that is where there's a gap in the wall.
        hole.0 +
        // Then, walk the hole up to (0, 0)
        hole.1 +
        // Walk right until you're right next to the target
        width - 2 +
        // Each move of the target is "shift target into hole", move hold around
        (width - 1) * 5 +
        // The last move doesn't need to shift the hole
        - 4;
    println!("Part 2: {}", steps);
}
