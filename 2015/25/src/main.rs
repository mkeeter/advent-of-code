use std::str::FromStr;

fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut itr = line
        .split(' ')
        .map(|w| w.trim_matches(|c| !char::is_numeric(c)))
        .filter_map(|w| i32::from_str(w).ok());

    let row = itr.next().unwrap();
    let col = itr.next().unwrap();
    assert!(itr.next() == None);

    let pos = std::iter::successors(Some((1, 1)), |&(r, c)| {
        if c == col && r == row {
            None
        } else if r == 1 {
            Some((c + 1, 1))
        } else {
            Some((r - 1, c + 1))
        }
    });
    let d = std::iter::successors(Some(20151125 as u64),
        |&i| { Some((i * 252533) % 33554393) });
    let i = pos.zip(d).last().unwrap().1;
    println!("Part 1: {:?}", i);
    println!("Part 2: ☆");
}
