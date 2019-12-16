use std::io::Read;
use std::cmp::min;

fn cumsum(input: &Vec<i32>) -> Vec<i32> {
    let mut output = vec![0; input.len() + 1];
    for (i, v) in input.iter().enumerate() {
        output[i + 1] = output[i] + v;
    }
    output
}

fn dft(scale: usize, csum: &Vec<i32>) -> i32 {
    assert!(scale > 0);
    let mut i = scale;
    let mut sign = true;
    let mut out = 0;
    while i < csum.len() {
        let d = csum[min(csum.len() - 1, i + scale - 1)] - csum[i - 1];
        if sign {
            out += d;
        } else {
            out -= d;
        }
        sign = !sign;
        i += scale * 2;
    }
    out.abs() % 10
}

fn main() {
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s).unwrap();

    let mut input = s.chars()
        .filter_map(|c| char::to_digit(c, 10))
        .map(|i| i as i32)
        .collect::<Vec<i32>>();

    for _ in 0..100 {
        let csum = cumsum(&input);
        input = (0..input.len())
            .map(|i| dft(i + 1, &csum))
            .collect::<Vec<i32>>();
    }

    print!("Part 1: ");
    for c in input[..8].iter() {
        print!("{}", c);
    }
    println!("");

    let size = input.len();
    let mut input = s.chars()
        .filter_map(|c| char::to_digit(c, 10))
        .map(|i| i as i32)
        .cycle()
        .take(size * 10000)
        .collect::<Vec<i32>>();
    let offset = input[..7].iter().fold(0, |acc, i| acc * 10 + i) as usize;

    for i in 0..100 {
        println!("{}", i);
        let csum = cumsum(&input);
        input = (0..input.len())
            .map(|i| dft(i + 1, &csum))
            .collect::<Vec<i32>>();
    }

    print!("Part 2: ");
    for c in input[offset..(offset + 8)].iter() {
        print!("{}", c);
    }
    println!("");
}
