use std::io::Read;

fn step(ring: &mut[u32], n: u32) {
    let current = ring[0];

    let a = ring[current as usize];
    let b = ring[a as usize];
    let c = ring[b as usize];
    let next = ring[c as usize];

    // Splice out the three elements
    ring[current as usize] = next;

    // Find the shuffle destination, wrapping as needed
    let mut dest = current;
    while dest == current || dest == a || dest == b || dest == c {
        dest -= 1;
        if dest == 0 {
            dest = n;
        }
    }

    let next = ring[dest as usize];
    ring[dest as usize] = a;
    ring[c as usize] = next;

    // Move the current pointer along one
    ring[0] = ring[current as usize];
}


fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut nums: Vec<u32> = input.chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    nums.push(nums[0]);

    ////////////////////////////////////////////////////////////////////////////
    // Part 1
    let mut ring = vec![0; 10];
    for a in nums[0..].windows(2) {
        ring[a[0] as usize] = a[1];
    }
    ring[0] = nums[0]; // Store the current cup in slot 0

    for _i in 0..100 {
        step(&mut ring[0..], 9);
    }

    print!("Part 1: ");
    let mut i = ring[1];
    while i != 1 {
        print!("{}", i);
        i = ring[i as usize];
    }
    println!();

    ////////////////////////////////////////////////////////////////////////////
    // Part 2
    const N: usize = 1_000_000;
    let mut ring = vec![0; N + 1];
    nums.pop();
    for i in 10..=N {
        nums.push(i as u32);
    }
    nums.push(nums[0]);
    for a in nums[0..].windows(2) {
        ring[a[0] as usize] = a[1];
    }
    println!("{:?}", ring.iter().max());
    ring[0] = nums[0]; // Store the current cup in slot 0

    for _i in 0..10_000_000 {
        step(&mut ring[0..], N as u32);
    }

    print!("Part 2: ");
    let a = ring[1];
    let b = ring[a as usize];
    println!("{} {} {}", a, b, a as u64 * b as u64);
}
