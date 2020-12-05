use std::io::BufRead;

fn bsp<I>(s: I, up: char, mut n: usize) -> usize
    where I: std::iter::Iterator<Item=char>
{
    s.zip(std::iter::repeat_with(move || { n /= 2; n }))
        .map(|(c, d)| if c == up { d } else { 0 })
        .sum()
}

fn pos(s: &str) -> (usize, usize) {
    (bsp(s.chars().take(7), 'B', 128), bsp(s.chars().skip(7), 'R', 8))
}

fn main() {
    let mut seats = std::io::stdin().lock().lines()
        .map(|line| pos(&line.unwrap()))
        .map(|(row, col)| row * 8 + col)
        .collect::<Vec<_>>();
    seats.sort();

    println!("Part 1: {}", seats.last().expect("Empty input"));

    let p2 = seats.iter().zip(seats.iter().skip(1))
        .find(|(a,b)| **b != *a + 1)
        .expect("Could not find missing seat");
    println!("Part 2: {}", p2.0 + 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pos() {
        assert_eq!(pos("BFFFBBFRRR"), (70, 7));
        assert_eq!(pos("FFFBBBFRRR"), (14, 7));
        assert_eq!(pos("BBFFBBFRLL"), (102, 4));
        assert_eq!(pos("FBFBBFFRLR"), (44, 5));
    }
}
