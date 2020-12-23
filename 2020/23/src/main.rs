use std::io::Read;

const N: u8 = 9;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut nums: Vec<u8> = input.chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();
    nums.push(nums[0]);

    let mut ring = [0; N as usize + 1];
    for a in nums[0..].windows(2) {
        ring[a[0] as usize] = a[1];
    }
    let mut current = nums[0]; // Store the current cup in slot 0

    for _i in 0..100 {
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
                dest = N;
            }
        }

        let next = ring[dest as usize];
        ring[dest as usize] = a;
        ring[c as usize] = next;

        // Move the pointer along one
        current = ring[current as usize];
    }
    print!("Part 1: ");
    let mut i = ring[1];
    while i != 1 {
        print!("{}", i);
        i = ring[i as usize];
    }
    println!();

}
