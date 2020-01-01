use std::collections::HashSet;

fn walk<I>(input: I) -> HashSet<(i32, i32)>
    where I: IntoIterator<Item=char>
{
    let mut pos = (0, 0);
    let mut seen = HashSet::new();
    seen.insert(pos);
    for b in input {
        let dpos = match b {
            '>' => ( 1,  0),
            '<' => (-1,  0),
            '^' => ( 0,  1),
            'v' => ( 0, -1),
            _ => panic!("Invalid char {}", b),
        };
        pos = (pos.0 + dpos.0, pos.1 + dpos.1);
        seen.insert(pos);
    }
    seen
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let itr = || input.trim().chars();

    println!("Part 1: {}", walk(itr()).len());

    let santa = walk(itr().step_by(2));
    let robosanta = walk(itr().skip(1).step_by(2));
    println!("Part 2: {}", santa.union(&robosanta).count());
}
