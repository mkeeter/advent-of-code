use std::str;

fn difference(a: u8, b: u8) -> i32 {
    ((a as i32) - (b as i32)).abs()
}

fn reacts(a: u8, b: u8) -> bool {
    difference(a, b) == difference(b'a', b'A')
}

fn fully_react(mut polymer: Vec<u8>) -> usize {
    loop {
        //let mut s = String::from_utf8(polymer.clone()).unwrap();
        //s.truncate(80);
        //println!("{}\n", s);

        let next = polymer
            .iter()
            .zip(polymer.iter().skip(1))
            .position(|(a, b)| reacts(*a, *b));

        match next {
            Some(i) => {
                polymer.remove(i);
                polymer.remove(i);
            }
            None => break,
        }
    }
    return polymer.len();
}

fn main() {
    let polymer = include_bytes!("../input")
        .iter()
        .filter(|c| (**c as char).is_alphabetic())
        .map(|c| c.clone())
        .collect::<Vec<u8>>();

    println!("part 1: {}", fully_react(polymer.clone()));
    println!("------------");

    let mut best = None;
    for c in b'A'..(b'Z' + 1) {
        let c = c as char;
        let mut shorter = polymer
            .iter()
            .filter(|e| (**e as char).to_ascii_uppercase() != c)
            .map(|p| p.clone())
            .collect::<Vec<u8>>();

        let score = fully_react(shorter);
        if best == None || score < best.unwrap() {
            best = Some(score);
        }
    }
    println!("Part 2: {}", best.unwrap());
}
