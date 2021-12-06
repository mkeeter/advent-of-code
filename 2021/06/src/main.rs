fn sim(fish: &[usize], days: usize) -> usize {
    let mut spawn = vec![0; days + 9];
    for &f in fish {
        spawn[f] += 1;
    }
    let mut pop = fish.len();
    for day in 1..days {
        pop += spawn[day];
        spawn[day + 7] += spawn[day];
        spawn[day + 9] += spawn[day];
    }
    pop
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let fish = input
        .trim()
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect::<Vec<usize>>();

    println!("Part 1: {}", sim(&fish, 80));
    println!("Part 2: {}", sim(&fish, 256));
}
