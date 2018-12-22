fn gotta_go_fast(players: usize, last: usize) -> usize {
    let mut prev = vec![0; last + 1];
    let mut next = vec![0; last + 1];
    let mut index = 0;

    let mut scores = vec![0; players];

    for i in 1..last {
        if i % 23 != 0 {
            index = next[index];

            let n = next[index];
            next[index] = i;
            next[i] = n;
            prev[i] = index;
            prev[n] = i;
        } else {
            let player = (i - 1) % players;
            scores[player] += i;
            for _ in 0..7 {
                index = prev[index];
            }
            scores[player] += index;
            next[prev[index]] = next[index];
            prev[next[index]] = prev[index];
        }
        index = next[index];
    }
    scores.iter().max().unwrap().clone()
}

fn main() {
    println!("{}", gotta_go_fast(9, 25));
    println!("{}", gotta_go_fast(411, 72059));
    println!("{}", gotta_go_fast(411, 72059 * 100));
}
