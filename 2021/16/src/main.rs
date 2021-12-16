struct BitReader {
    pos: usize,
    data: Vec<u32>,
}
impl BitReader {
    fn read(&mut self, mut amt: usize) -> u64 {
        let mut out: u64 = 0;
        while amt > 0 {
            let bit = 31 - (self.pos % 32);
            let size = amt.min(bit + 1);
            let shift = bit + 1 - size;
            let mask = (1 << size) - 1;

            out <<= size;
            out |= ((self.data[self.pos / 32] as u64) >> shift) & mask;
            self.pos += size;
            amt -= size;
        }
        out
    }
}

#[derive(Debug)]
struct Packet {
    version: u8,
    typeid: u8,
    body: Body,
}
#[derive(Debug)]
enum Body {
    Value(u64),
    Operator(Vec<Packet>),
}
impl Packet {
    fn from(reader: &mut BitReader) -> Self {
        let version = reader.read(3) as u8;
        let typeid = reader.read(3) as u8;
        if typeid == 4 {
            let mut value = 0;
            loop {
                let v = reader.read(5);
                value = (value << 4) | (v & 0b1111);
                if (v & 0b10000) == 0 {
                    break;
                }
            }
            Packet {
                version,
                typeid,
                body: Body::Value(value),
            }
        } else {
            let mut ops = vec![];
            match reader.read(1) {
                0 => {
                    let len = reader.read(15);
                    let end = len as usize + reader.pos;
                    while reader.pos != end {
                        ops.push(Packet::from(reader));
                    }
                }
                1 => {
                    for _i in 0..reader.read(11) {
                        ops.push(Packet::from(reader));
                    }
                }
                _ => panic!("invalid length type id"),
            }
            Packet {
                version,
                typeid,
                body: Body::Operator(ops),
            }
        }
    }
    fn version_sum(&self) -> u64 {
        self.version as u64
            + match &self.body {
                Body::Value(_) => 0,
                Body::Operator(ops) => ops.iter().map(|v| v.version_sum()).sum(),
            }
    }
    fn value(&self) -> u64 {
        match &self.body {
            Body::Value(v) => {
                assert!(self.typeid == 4);
                *v
            }
            Body::Operator(ops) => match self.typeid {
                0 => ops.iter().map(|op| op.value()).sum(),
                1 => ops.iter().map(|op| op.value()).product(),
                2 => ops.iter().map(|op| op.value()).min().unwrap(),
                3 => ops.iter().map(|op| op.value()).max().unwrap(),
                5 => (ops[0].value() > ops[1].value()) as u64,
                6 => (ops[0].value() < ops[1].value()) as u64,
                7 => (ops[0].value() == ops[1].value()) as u64,
                _ => panic!("Invalid opcode {}", self.typeid),
            },
        }
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let data = input
        .trim()
        .as_bytes()
        .chunks(8)
        .map(|c| {
            let s = std::str::from_utf8(c).unwrap();
            u32::from_str_radix(s, 16).unwrap() << (4 * (8 - c.len()))
        })
        .collect::<Vec<u32>>();
    let mut reader = BitReader { pos: 0, data };

    let root = Packet::from(&mut reader);
    println!("Part 1: {}", root.version_sum());
    println!("Part 2: {}", root.value());
}
