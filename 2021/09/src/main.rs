use std::io::BufRead;

fn main() {
    let heights = std::io::stdin()
        .lock()
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c as u32 - '0' as u32)
                .collect()
        })
        .collect::<Vec<Vec<u32>>>();

    let mut low = Vec::new();
    for (j, row) in heights.iter().enumerate() {
        for (i, v) in row.iter().enumerate() {
            if [(0, 1), (0, -1), (1, 0), (-1, 0)].iter().all(|(dx, dy)| {
                heights
                    .get((j as i64 + dy) as usize)
                    .and_then(|row| row.get((i as i64 + dx) as usize))
                    .filter(|&w| w <= v)
                    .is_none()
            }) {
                low.push((j, i));
            }
        }
    }
    let score: u32 = low.iter().map(|&(j, i)| heights[j][i] + 1).sum();
    println!("Part 1: {}", score);
}
