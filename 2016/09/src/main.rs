use std::io::Read;
use std::str::FromStr;

#[derive(Debug)]
enum Chunk {
    Chars(usize),
    Marker(usize, usize),
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

fn expand(s: &str) -> String {
    enum State {
        Normal,
        ReadBlockSize,
        ReadBlockCount,
        ReadBlock,
    };
    use State::*;

    let mut state = Normal;
    let mut out = Vec::new();
    let mut tmp = Vec::new();
    let mut block_size = 0;
    let mut block_count = 0;

    for c in s.chars() {
        match state {
            Normal => {
                if (c) == '(' {
                    state = ReadBlockSize;
                    block_size = 0;
                } else {
                    out.push(c);
                }
            }
            ReadBlockSize => {
                if c == 'x' {
                    state = ReadBlockCount;
                    block_count = 0;
                } else {
                    block_size = block_size * 10 + c.to_digit(10).unwrap();
                }
            }
            ReadBlockCount => {
                if c == ')' {
                    state = ReadBlock;
                    tmp.clear();
                } else {
                    block_count = block_count * 10 + c.to_digit(10).unwrap();
                }
            }
            ReadBlock => {
                tmp.push(c);
                block_size -= 1;
                if block_size == 0 {
                    for _i in 0..block_count {
                        for t in tmp.iter() {
                            out.push(*t);
                        }
                    }
                    state = Normal;
                }
            }
        }
    }
    out.iter().collect()
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    input = input.trim().to_owned();

    println!("{:?}", parse(input.clone()));

    input = expand(&input);
    println!("{}", input.len());
}
