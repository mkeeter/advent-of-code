use std::io::Read;

type Password = [u8; 8];

fn next(pw: &Password) -> Password {
    let mut carry = 1;
    let mut out = [0; 8];
    for (i, c) in pw.iter().enumerate().rev() {
        let mut next = *c + carry;
        if next > b'z' {
            next -= b'z' - b'a' + 1;
            carry = 1;
        } else {
            carry = 0;
        }
        out[i] = next;
    }
    out
}

fn check(pw: &Password) -> bool {
    pw.windows(3)   // Three increasing characters
        .any(|w| w[1] == w[0] + 1 && w[2] == w[1] + 1)
    &&
    !pw.iter()      // No characters from the blacklist
        .any(|&c| c == b'i' || c == b'o' || c == b'l')
    &&
    pw.windows(2)   // Two non-overlapping pairs
        .enumerate()
        .filter(|w| w.1[0] == w.1[1])
        .map(|i| i.0)
        .collect::<Vec<usize>>()
        .windows(2)
        .any(|q| q[1] - q[0] > 1)
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut pw = [0; 8];
    for (i, c) in input.chars().enumerate().take(8) {
        pw[i] = c as u8;
    }

    let mut itr = std::iter::successors(Some(pw), |c| Some(next(c)))
        .filter(check);

    for i in 1..=2 {
        print!("Part {}: ", i);
        let sol = itr.next().unwrap();
        for i in sol.iter() {
            print!("{}", *i as char);
        }
        println!();
    }
}
