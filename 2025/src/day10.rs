use std::{
    fmt::Write as _,
    io::Write as _,
    process::{Command, Stdio},
};

struct Machine {
    expected: u16,
    /// Each button, as a bitmask of the lights that it toggles
    buttons: Vec<u16>,
    joltage: Vec<usize>,
    width: usize,
}

pub fn solve(s: &str) -> (u64, u64) {
    let mut machines = vec![];
    for line in s.lines() {
        let mut iter = line.split(' ');
        let lights = iter.next().unwrap();
        let mut expected = 0;
        for c in lights.as_bytes()[1..lights.len() - 1].iter() {
            expected = expected << 1
                | match c {
                    b'.' => 0,
                    b'#' => 1,
                    c => panic!("invalid light character '{c}'"),
                }
        }
        let width = lights.len() - 2; // skipping `[]` characters
        let mut buttons = vec![];
        let mut joltage = None;
        for set in iter {
            let is_joltage = if set.starts_with('(') {
                false
            } else if set.starts_with('{') {
                true
            } else {
                panic!("invalid set start character in '{set}'");
            };
            let set = set.trim_matches(&['{', '}', '(', ')']);
            let vs = set
                .split(',')
                .map(|i| i.parse().unwrap())
                .collect::<Vec<usize>>();
            if is_joltage {
                joltage = Some(vs)
            } else {
                let mut out = 0;
                for v in vs {
                    out |= 1u16 << (width - v - 1);
                }
                buttons.push(out);
            }
        }
        machines.push(Machine {
            expected,
            buttons,
            width,
            joltage: joltage.expect("missing joltage"),
        })
    }
    let mut part1 = 0;
    for m in &machines {
        // Iterate over all button combinations
        let mut best = u64::MAX;
        for i in 0..(1usize << m.buttons.len()) {
            let out = m
                .buttons
                .iter()
                .enumerate()
                .filter(|(j, _b)| i & (1 << j) != 0)
                .fold(0, |a, (_j, b)| a ^ b);
            if out == m.expected {
                best = best.min(u64::from(i.count_ones()));
            }
        }
        assert!(best < u64::MAX);
        part1 += best;
    }
    let mut smt = String::new();
    for (k, m) in machines.iter().enumerate() {
        let mut light_to_buttons = vec![];
        light_to_buttons.resize_with(m.width, Vec::new);
        for (i, b) in m.buttons.iter().enumerate() {
            writeln!(&mut smt, "(declare-const m{k}_b{i} Int)").unwrap();
            writeln!(&mut smt, "(assert (>= m{k}_b{i} 0))").unwrap();
            for j in 0..m.width {
                if b & (1 << j) != 0 {
                    light_to_buttons[m.width - j - 1].push(i);
                }
            }
        }
        writeln!(&mut smt, "(declare-const m{k}_sum Int)").unwrap();
        assert_eq!(light_to_buttons.len(), m.joltage.len());
        for (buttons, joltage) in light_to_buttons.iter().zip(m.joltage.iter())
        {
            let sum = buttons
                .iter()
                .map(|b| format!("m{k}_b{b}"))
                .collect::<Vec<_>>()
                .join(" ");
            writeln!(&mut smt, "(assert (= {joltage} (+ {sum})))").unwrap();
        }
        let all_buttons = (0..m.buttons.len())
            .map(|i| format!("m{k}_b{i}"))
            .collect::<Vec<_>>()
            .join(" ");
        writeln!(&mut smt, "(assert (= m{k}_sum (+ {all_buttons})))").unwrap();
        writeln!(&mut smt, "(minimize m{k}_sum)").unwrap();
        writeln!(&mut smt).unwrap();
    }
    writeln!(&mut smt, "(check-sat)").unwrap();
    let sum = (0..machines.len())
        .map(|k| format!("m{k}_sum"))
        .collect::<Vec<_>>()
        .join(" ");
    writeln!(&mut smt, "(eval (+ {sum}))").unwrap();

    let mut z3 = Command::new("z3")
        .arg("-in")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to call `z3`; is it installed?");

    let mut stdin = z3.stdin.take().expect("Failed to open stdin");
    stdin
        .write_all(smt.as_bytes())
        .expect("failed to write to z3");
    drop(stdin);
    let output = z3.wait_with_output().expect("failed to read stdout");
    let out = String::from_utf8(output.stdout).unwrap();
    let mut iter = out.lines();
    assert_eq!(iter.next().unwrap(), "sat");
    let part2: u64 = iter.next().unwrap().parse().unwrap();
    assert!(iter.next().is_none());
    (part1, part2)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let s = indoc::indoc! {"
            [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
            [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
            [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
        "};
        let (a, b) = solve(s);
        assert_eq!(a, 7);
        assert_eq!(b, 33);
    }
}
