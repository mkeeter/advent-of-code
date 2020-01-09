use std::io::Read;
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
enum Chunk {
    Chars(usize),
    Marker(usize, usize),
}

impl Chunk {
    fn len(&self) -> usize {
        match *self {
            Self::Chars(u) => u,
            Self::Marker(mut a, mut b) => {
                let mut len = 3;
                while a > 0 {
                    len += 1;
                    a /= 10;
                }
                while b > 0 {
                    len += 1;
                    b /= 10;
                }
                len
            },
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

        let mut itr = marker
            .split('x')
            .filter_map(|i| usize::from_str(i).ok());
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

fn expand(s: &[Chunk], recurse: bool) -> usize {
    use Chunk::*;

    let mut i = 0;
    let mut out = 0;
    while i < s.len() {
        match s[i] {
            Chars(s) => {
                out += s;
            },
            Marker(size, repeat) => {
                let mut size: usize = size;
                let mut tmp = Vec::new();
                while size > 0 {
                    i += 1;
                    match s[i] {
                        Chars(s) => {
                            if s > size {
                                tmp.push(Chars(size));
                                out += s - size;
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
                out += repeat * if recurse {
                    expand(&tmp, recurse)
                } else {
                    tmp.iter().map(|c| c.len()).sum::<usize>()
                };
            }
        }
        i += 1;
    }
    out
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let input = parse(input.trim().to_string());
    println!("Part 1: {}", expand(&input, false));
    println!("Part 2: {}", expand(&input, true));
}
