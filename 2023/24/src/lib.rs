use parse_display::FromStr;
use std::fmt::Write;
use std::io::Write as _;

#[derive(Copy, Clone, Debug)]
struct Vector {
    x: i64,
    y: i64,
    z: i64,
}

impl std::str::FromStr for Vector {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(',').map(|v| v.trim().parse::<_>().unwrap());
        let x = iter.next().unwrap();
        let y = iter.next().unwrap();
        let z = iter.next().unwrap();
        Ok(Self { x, y, z })
    }
}

#[derive(Copy, Clone, Debug, FromStr)]
#[display("{pos} @ {vel}")]
struct Hail {
    pos: Vector,
    vel: Vector,
}

pub fn solve(s: &str) -> (String, String) {
    let stones: Vec<Hail> =
        s.lines().map(|s| s.parse::<Hail>().unwrap()).collect();
    let mut hits = 0;

    let bounds_min = 200000000000000.0;
    let bounds_max = 400000000000000.0;

    for (i, &a) in stones.iter().enumerate() {
        for &b in &stones[..i] {
            let num =
                b.vel.x * (a.pos.y - b.pos.y) - b.vel.y * (a.pos.x - b.pos.x);
            let denom = a.vel.x * b.vel.y - b.vel.x * a.vel.y;
            if denom == 0 {
                continue;
            }
            let ta = num as f64 / denom as f64;
            let tb = (a.pos.x as f64 + ta * a.vel.x as f64 - b.pos.x as f64)
                / b.vel.x as f64;

            if ta < 0.0 || tb < 0.0 {
                continue;
            }

            let x = a.pos.x as f64 + a.vel.x as f64 * ta;
            let y = a.pos.y as f64 + a.vel.y as f64 * ta;
            if x >= bounds_min
                && x <= bounds_max
                && y >= bounds_min
                && y <= bounds_max
            {
                hits += 1;
            }
        }
    }
    let p1 = hits;

    let mut f = String::new();
    for v in ["x", "y", "z"] {
        writeln!(
            f,
            "(declare-const r{v} Int)
             (declare-const d{v} Int)"
        )
        .unwrap();
    }
    for (i, s) in stones.iter().enumerate() {
        let (x, y, z) = (s.pos.x, s.pos.y, s.pos.z);
        let (dx, dy, dz) = (s.vel.x, s.vel.y, s.vel.z);
        writeln!(
            f,
            "(declare-const t{i} Int)
             (assert (> t{i} 0))
             (assert (= (+ rx (* dx t{i})) (+ {x} (* {dx} t{i}))))
             (assert (= (+ ry (* dy t{i})) (+ {y} (* {dy} t{i}))))
             (assert (= (+ rz (* dz t{i})) (+ {z} (* {dz} t{i}))))"
        )
        .unwrap();
    }
    writeln!(
        f,
        "(check-sat)
         (eval (+ rx ry rz))"
    )
    .unwrap();

    use std::process::{Command, Stdio};
    let mut z3 = Command::new("z3")
        .arg("-in")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process");

    let mut stdin = z3.stdin.take().expect("Failed to open stdin");
    stdin
        .write_all(f.as_bytes())
        .expect("failed to write to z3");
    drop(stdin);
    let output = z3.wait_with_output().expect("Failed to read stdout");
    let out = String::from_utf8(output.stdout).unwrap();
    let p2 = out.lines().nth(1).unwrap().parse::<usize>().unwrap();

    (p1.to_string(), p2.to_string())
}
