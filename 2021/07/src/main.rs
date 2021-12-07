fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut crabs = input
        .trim()
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect::<Vec<i64>>();

    crabs.sort_unstable();
    let center = crabs[crabs.len() / 2];
    let fuel: i64 = crabs.iter().map(|c| (c - center).abs()).sum();
    println!("Part 1: {}", fuel);

    let min = *crabs.iter().min().unwrap();
    let max = *crabs.iter().max().unwrap();
    let (_center, fuel) = (min..=max)
        .map(|center| {
            (
                center,
                crabs
                    .iter()
                    .map(|c| {
                        let d = (c - center).abs();
                        ((d + 1) * d) / 2
                    })
                    .sum::<i64>(),
            )
        })
        .min_by_key(|k| k.1)
        .unwrap();
    println!("Part 2: {}", fuel)
}
