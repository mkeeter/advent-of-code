use std::io::Read;

fn run(input: &[u32], n: usize) -> u32 {
    let mut history = vec![0; n];
    let mut next: u32 = 0;
    for i in 0..n {
        let speak = next;

        // Where wast thou when I laid the foundations of the Earth?
        if i < input.len() {
            next = input[i];
        } else if history[next as usize] == 0 {
            next = 0;
        } else {
            next = i as u32 - history[next as usize];
        }

        // Declare, if thou hast understanding.
        history[speak as usize] = i as u32;
    }
    next
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let input: Vec<u32> = input.split(",").flat_map(|s| s.parse()).collect();

    println!("Part 1: {}", run(&input, 2020));
    println!("Part 2: {}", run(&input, 30000000));
}
