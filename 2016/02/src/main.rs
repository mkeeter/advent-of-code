use std::collections::HashMap;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let run = |keypad: &str| {
        let mut keys: HashMap<(i32, i32), char> = HashMap::new();
        let mut pos = (0, 0);
        for (y, line) in keypad.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c != ' ' {
                    let p = (x as i32 / 2, y as i32);
                    keys.insert(p, c);
                    if c == '5' {
                        pos = p;
                    }
                }
            }
        }

        for line in input.lines() {
            for b in line.chars() {
                let next = match b {
                    'U' => (pos.0, pos.1 - 1),
                    'D' => (pos.0, pos.1 + 1),
                    'R' => (pos.0 + 1, pos.1),
                    'L' => (pos.0 - 1, pos.1),
                    _ => panic!("Invalid character"),
                };
                if keys.get(&next).is_some() {
                    pos = next;
                }
            }
            print!("{}", keys.get(&pos).unwrap())
        }
    };

    print!("Part 1: ");
    run(concat!("1 2 3\n", "4 5 6\n", "7 8 9"));
    println!();

    print!("Part 2: ");
    run(concat!(
        "    1    \n",
        "  2 3 4  \n",
        "5 6 7 8 9\n",
        "  A B C  \n",
        "    D    "
    ));
    println!();
}
