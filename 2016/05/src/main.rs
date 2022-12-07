fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().to_string();

    let hashes = || {
        std::iter::successors(Some(1), |i| Some(i + 1))
            .map(|i| input.clone() + &i.to_string())
            .map(md5::compute)
            .filter(|m| m[0] == 0 && m[1] == 0 && m[2] >> 4 == 0)
    };

    let part1 = hashes()
        .take(8)
        .map(|m| std::char::from_digit(m[2] as u32 & 0xF, 16).unwrap())
        .collect::<String>();
    println!("Part 1: {}", part1);

    let mut part2 = [None; 8];
    for h in hashes() {
        if part2.iter().all(|i| i.is_some()) {
            break;
        }
        let pos = h[2] as usize & 0xF;
        if pos < 8 && part2[pos].is_none() {
            let c = std::char::from_digit((h[3] >> 4) as u32, 16).unwrap();
            part2[pos] = Some(c);
        }
    }
    let part2 = part2.iter().map(|i| i.unwrap()).collect::<String>();
    println!("Part 2: {}", part2);
}
