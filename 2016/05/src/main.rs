fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().to_string();

    let hashes = || std::iter::successors(Some(1), |i| Some(i + 1))
        .map(|i| input.clone() + &i.to_string())
        .map(md5::compute);

    let part1 = hashes()
        .filter(|m| m[0] == 0 && m[1] == 0 && m[2] >> 4 == 0)
        .take(8)
        .map(|m| std::char::from_digit(m[2] as u32 & 0xF, 16).unwrap())
        .collect::<String>();
    println!("Part 1: {}", part1);
}
