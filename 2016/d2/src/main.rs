use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let mut x : usize = 1;
    let mut y : usize = 1;

    let beep = "  1  "
               " 234 "
               "56789"
               " ABC "
               "  D  ";

    let numpad = [[7, 8, 9], [4, 5, 6], [1, 2, 3]];

    for line in buffer.lines() {
        for b in line.as_bytes() {
            match *b as char {
                'U' => y = if y == 2 { 2 } else {y + 1},
                'D' => y = if y == 0 { 0 } else {y - 1},
                'R' => x = if x == 2 { 2 } else {x + 1},
                'L' => x = if x == 0 { 0 } else {x - 1},
                _ => panic!("wargarble"),
            }
        }
        print!("{}", numpad[y][x]);
    }
}
