use std::io::BufRead;

#[derive(Default, Debug)]
struct Board {
    grid: [u8; 25],
    bits: u32,
    done: bool,
}

impl Board {
    /// Marks the given number in the grid, return the score if this results
    /// in a winning board.
    fn mark(&mut self, i: u8) -> Option<u32> {
        assert!(!self.done);
        if let Some(bit) = self.grid.iter().position(|j| i == *j) {
            self.bits |= 1 << bit;
        }
        if self.winning() {
            self.done = true;
            Some(self.score() * i as u32)
        } else {
            None
        }
    }
    fn winning(&self) -> bool {
        [
            (0b0000000000000000000011111, 5),
            (0b0000100001000010000100001, 1),
        ]
        .iter()
        .flat_map(|(mask, stride)| (0..5).map(move |i| mask << (i * stride)))
        .any(|mask| self.bits & mask == mask)
    }
    fn score(&self) -> u32 {
        (0..25)
            .filter(|bit| (self.bits & (1 << bit)) == 0)
            .map(|bit| self.grid[bit] as u32)
            .sum()
    }
}

fn main() {
    let lines = std::io::stdin()
        .lock()
        .lines()
        .map(|line| {
            line.unwrap()
                .split(&[',', ' '][..])
                .filter_map(|n| n.parse().ok())
                .collect()
        })
        .collect::<Vec<Vec<u8>>>();

    let mut boards = vec![];
    for chunk in lines[1..].split(|v| v.is_empty()).filter(|c| !c.is_empty()) {
        let mut board = Board::default();
        chunk
            .iter()
            .flat_map(|line| line.iter())
            .enumerate()
            .for_each(|(i, j)| board.grid[i] = *j);
        boards.push(board);
    }
    let mut wins = Vec::new();
    for num in lines[0].iter() {
        wins.extend(
            boards
                .iter_mut()
                .filter(|b| !b.done)
                .filter_map(|b| b.mark(*num)),
        );
    }
    println!("Part 1: {}", wins[0]);
    println!("Part 2: {}", wins.last().unwrap());
}
