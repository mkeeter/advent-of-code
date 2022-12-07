#[derive(Debug)]
struct Tree {
    nodes: Vec<Tree>,
    metadata: Vec<usize>,
}

impl Tree {
    fn from_iter<I>(s: &mut I) -> Self
    where
        I: Iterator<Item = usize>,
    {
        let num_children = s.next().unwrap();
        let num_meta = s.next().unwrap();

        Tree {
            nodes: (0..num_children).map(|_| Self::from_iter(s)).collect(),
            metadata: s.take(num_meta).collect(),
        }
    }

    fn basic_score(&self) -> usize {
        self.metadata.iter().sum::<usize>()
            + self.nodes.iter().map(|b| b.basic_score()).sum::<usize>()
    }

    fn fancy_score(&self) -> usize {
        if self.nodes.is_empty() {
            self.metadata.iter().sum()
        } else {
            self.metadata
                .iter()
                .filter_map(|i| i.checked_sub(1))
                .filter_map(|i| self.nodes.get(i))
                .map(Tree::fancy_score)
                .sum()
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

fn main() {
    let a = include_str!("../input")
        .split(" ")
        .map(str::trim)
        .map(str::parse)
        .filter_map(Result::ok)
        .collect::<Vec<usize>>();

    let t = Tree::from_iter(&mut a.into_iter());
    println!("{}", t.basic_score());
    println!("{}", t.fancy_score());
}
