fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().to_string();

    let hashes = || {
        std::iter::successors(Some(1), |i| Some(i + 1))
            .map(|i| input.clone() + &i.to_string())
            .map(md5::compute)
    };

    let n = hashes()
        .take_while(|m| m[0] != 0 || m[1] != 0 || m[2] >> 4 != 0)
        .count();
    println!("Part 1: {}", n + 1);

    let m = hashes()
        .take_while(|m| m[0] != 0 || m[1] != 0 || m[2] != 0)
        .count();
    println!("Part 2: {}", m + 1);
}
