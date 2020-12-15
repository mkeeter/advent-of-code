use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let input: Vec<usize> = input.split(",").map(|s| s.parse().unwrap()).collect();

    let mut history = [0; 65536];
    let mut next = 0;
    for i in 0..2020 {
        let speak = next;

        if i < input.len() {
            next = input[i];
        } else if history[next] == 0 {
            next = 0;
        } else {
            next = i - history[next];
        }

        history[speak] = i;
    }

    println!("{}", next);

    let mut history = vec![0; 30000000];
    let mut next = 0;
    for i in 0..30000000 {
        let speak = next;

        if i < input.len() {
            next = input[i];
        } else if history[next] == 0 {
            next = 0;
        } else {
            next = i - history[next];
        }

        history[speak] = i;
    }
    println!("{}", next);
}
