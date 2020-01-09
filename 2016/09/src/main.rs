use std::io::Read;
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
enum Chunk {
    Chars(usize),
    Marker(usize, usize),
}

impl Chunk {
    fn len(&self) -> usize {
        match self {
            Self::Chars(u) => *u,
            Self::Marker(a, b) => format!("({}x{})", a, b).len(),
        }
    }
}


fn parse(mut s: String) -> Vec<Chunk> {
    let mut out = Vec::new();
    while let Some(i) = s.find('(') {
        if i > 0 {
            out.push(Chunk::Chars(i));
        }
        let mut marker = s.split_off(i + 1);
        let j = marker.find(')').unwrap();
        let mut next = marker.split_off(j);

        println!("{}", marker);
        let mut itr = marker.split('x').filter_map(|i| usize::from_str(i).ok());
        let size = itr.next().unwrap();
        let repeat = itr.next().unwrap();
        out.push(Chunk::Marker(size, repeat));
        s = next.split_off(1); // drop the ')'
    }
    let i = s.len();
    if i > 0 {
        out.push(Chunk::Chars(i));
    }
    out
}

fn expand(s: &[Chunk]) -> Vec<Chunk> {
    use Chunk::*;

    let mut i = 0;
    let mut out = Vec::new();
    while i < s.len() {
        match s[i] {
            Chars(s) => {
                out.push(Chars(s));
                i += 1;
            },
            Marker(size, repeat) => {
                let mut size: usize = size;
                let mut tmp = Vec::new();
                let mut next = None;
                while size > 0 {
                    i += 1;
                    match s[i] {
                        Chars(s) => {
                            if s > size {
                                tmp.push(Chars(size));
                                next = Some(Chars(s - size));
                                break;
                            } else {
                                tmp.push(Chars(s));
                            }
                        },
                        Marker(a,b) => {
                            tmp.push(Marker(a, b));
                        }
                    }
                    size -= s[i].len();
                }
                for _r in 0..repeat {
                    for t in tmp.iter() {
                        out.push(*t);
                    }
                }
                if let Some(n) = next {
                    out.push(n);
                }
                i += 1;
            }
        }
    }
    out
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let parsed = parse(input.trim().to_string());
    let e = expand(&parsed);
    println!("{:?}", e);
}
