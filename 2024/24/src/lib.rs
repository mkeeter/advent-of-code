use std::collections::HashMap;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Name([char; 3]);

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.0[0], self.0[1], self.0[2])
    }
}

impl Name {
    fn new(value: &str) -> Self {
        if value.len() != 3 {
            panic!("invalid name {value}")
        } else {
            let mut iter = value.chars();
            Name(std::array::from_fn(|_| iter.next().unwrap()))
        }
    }

    fn index(&self) -> Option<u32> {
        if self.0[1].is_ascii_digit() && self.0[2].is_ascii_digit() {
            Some(
                (self.0[1] as u32 - '0' as u32) * 10
                    + (self.0[2] as u32 - '0' as u32),
            )
        } else {
            None
        }
    }

    fn starts_with(&self, c: char) -> bool {
        self.0[0] == c
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Opcode {
    And,
    Or,
    Xor,
}

impl std::fmt::Display for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::And => "AND",
                Self::Or => "OR",
                Self::Xor => "XOR",
            }
        )
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Gate {
    a: Name,
    b: Name,
    op: Opcode,
}

impl std::fmt::Display for Gate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.a, self.b, self.op)
    }
}

impl Gate {
    fn new(a: Name, b: Name, op: Opcode) -> Self {
        Gate {
            a: a.min(b),
            b: a.max(b),
            op,
        }
    }
}

#[derive(Debug)]
enum Error {
    CouldNotFind(Gate),
    BadOutput(Name, Name),
}

#[derive(Default)]
struct Gates {
    gates: HashMap<Gate, Name>,
    swaps: Vec<(Name, Name)>,
}

impl Gates {
    fn new() -> Self {
        Self::default()
    }
    fn insert(&mut self, g: Gate, out: Name) {
        self.gates.insert(g, out);
    }

    fn run_until_stable(&self, mut values: HashMap<Name, bool>) -> u64 {
        let mut active_gates =
            self.gates.iter().map(|(g, n)| (*g, *n)).collect::<Vec<_>>();
        while !active_gates.is_empty() {
            let mut next = vec![];
            for (g, out) in active_gates {
                let (Some(va), Some(vb)) = (values.get(&g.a), values.get(&g.b))
                else {
                    next.push((g, out));
                    continue;
                };
                let vo = match g.op {
                    Opcode::Or => va | vb,
                    Opcode::And => va & vb,
                    Opcode::Xor => va ^ vb,
                };
                values.insert(out, vo);
            }
            active_gates = next;
        }
        let mut out = 0u64;
        for (k, v) in values {
            if k.starts_with('z') && v {
                out |= 1 << k.index().unwrap();
            }
        }
        out
    }

    fn find(&self, a: Name, b: Name, op: Opcode) -> Result<Name, Error> {
        let g = Gate::new(a, b, op);
        self.gates.get(&g).cloned().ok_or(Error::CouldNotFind(g))
    }

    fn swap_inner(&mut self, a: Name, b: Name) {
        for v in self.gates.values_mut() {
            if *v == a {
                *v = b;
            } else if *v == b {
                *v = a;
            }
        }
    }

    fn swap(&mut self, a: Name, b: Name) {
        self.swap_inner(a, b);
        self.swaps.push((a, b));
    }

    fn unswap(&mut self, a: Name, b: Name) {
        self.swap_inner(a, b);
        self.swaps.retain(|s| *s != (a, b));
    }

    fn width(&self) -> u32 {
        self.gates
            .values()
            .filter(|o| o.starts_with('z'))
            .flat_map(|o| o.index())
            .max()
            .unwrap()
    }

    fn solve(&mut self) {
        let outputs: Vec<Name> = self.gates.values().cloned().collect();
        'outer: while let Err((i, e)) = self.check() {
            let g = match e {
                Error::CouldNotFind(g) => g,
                Error::BadOutput(a, b) => {
                    self.swap(a, b);
                    continue 'outer;
                }
            };
            // Try swapping gate inputs and see if we do better
            for target in [g.a, g.b] {
                for &o in &outputs {
                    if target != o {
                        self.swap(target, o);
                        match self.check() {
                            Ok(()) => {
                                return; // we did it!
                            }
                            Err((j, _e)) if j > i => {
                                continue 'outer; // we got better
                            }
                            Err(..) => (), // we didn't get better
                        }
                        self.unswap(target, o);
                    }
                }
            }
            panic!("could not find swap");
        }
    }

    fn check(&self) -> Result<(), (u32, Error)> {
        let width = self.width();
        let mut carry = None;
        for i in 0..width {
            let a = Name::new(&format!("x{i:02}"));
            let b = Name::new(&format!("y{i:02}"));
            let expected_sum = Name::new(&format!("z{i:02}"));

            let e = |err| (i, err); // tag the error with our width
            if let Some(c) = carry {
                let vsum = self.find(a, b, Opcode::Xor).map_err(e)?;
                let vcarry = self.find(a, b, Opcode::And).map_err(e)?;
                let sum = self.find(vsum, c, Opcode::Xor).map_err(e)?;

                // Easy fix:
                if sum != expected_sum {
                    return Err(e(Error::BadOutput(sum, expected_sum)));
                }

                let scarry = self.find(c, vsum, Opcode::And).map_err(e)?;
                carry = Some(self.find(vcarry, scarry, Opcode::Or).map_err(e)?);
            } else {
                let sum = self.find(a, b, Opcode::Xor).unwrap();
                if sum != expected_sum {
                    return Err(e(Error::BadOutput(sum, expected_sum)));
                }
                carry = Some(self.find(a, b, Opcode::And).map_err(e)?);
            }
        }
        Ok(())
    }
}

pub fn solve(s: &str) -> (u64, String) {
    let mut inputs = true;
    let mut values = HashMap::new();
    let mut gates = Gates::new();
    for line in s.lines() {
        if line.is_empty() {
            inputs = false;
        } else if inputs {
            let name = Name::new(&line[..3]);
            let value = match line.as_bytes()[5] {
                b'0' => false,
                b'1' => true,
                c => panic!("invalid value {c}"),
            };
            values.insert(name, value);
        } else {
            let mut iter = line.split_ascii_whitespace();
            let a = Name::new(iter.next().unwrap());
            let op = match iter.next().unwrap() {
                "AND" => Opcode::And,
                "OR" => Opcode::Or,
                "XOR" => Opcode::Xor,
                v => panic!("invalid opcode {v}"),
            };
            let b = Name::new(iter.next().unwrap());
            let arrow = iter.next().unwrap();
            assert_eq!(arrow, "->");
            let out = Name::new(iter.next().unwrap());
            gates.insert(Gate::new(a, b, op), out);
        }
    }
    let p1 = gates.run_until_stable(values);

    gates.solve();
    gates.check().unwrap();
    let mut swapped = gates
        .swaps
        .into_iter()
        .flat_map(|(a, b)| [a, b].into_iter())
        .collect::<Vec<_>>();
    swapped.sort();
    let mut out = String::new();
    for (i, g) in swapped.into_iter().enumerate() {
        if i > 0 {
            out += ",";
        }
        out += &format!("{g}");
    }
    (p1, out)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        const EXAMPLE_1: &str = indoc::indoc! {"
            x00: 1
            x01: 0
            x02: 1
            x03: 1
            x04: 0
            y00: 1
            y01: 1
            y02: 1
            y03: 1
            y04: 1

            ntg XOR fgs -> mjb
            y02 OR x01 -> tnw
            kwq OR kpj -> z05
            x00 OR x03 -> fst
            tgd XOR rvg -> z01
            vdt OR tnw -> bfw
            bfw AND frj -> z10
            ffh OR nrd -> bqk
            y00 AND y03 -> djm
            y03 OR y00 -> psh
            bqk OR frj -> z08
            tnw OR fst -> frj
            gnj AND tgd -> z11
            bfw XOR mjb -> z00
            x03 OR x00 -> vdt
            gnj AND wpb -> z02
            x04 AND y00 -> kjc
            djm OR pbm -> qhw
            nrd AND vdt -> hwm
            kjc AND fst -> rvg
            y04 OR y02 -> fgs
            y01 AND x02 -> pbm
            ntg OR kjc -> kwq
            psh XOR fgs -> tgd
            qhw XOR tgd -> z09
            pbm OR djm -> kpj
            x03 XOR y03 -> ffh
            x00 XOR y04 -> ntg
            bfw OR bqk -> z06
            nrd XOR fgs -> wpb
            frj XOR qhw -> z04
            bqk OR frj -> z07
            y03 OR x01 -> nrd
            hwm AND bqk -> z03
            tgd XOR rvg -> z12
            tnw OR pbm -> gnj
        "};
        assert_eq!(solve(EXAMPLE_1).0, 2024);
    }
}
