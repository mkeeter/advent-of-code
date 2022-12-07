use anyhow::{anyhow, bail, Context, Result};
use std::collections::BTreeMap;
use std::io::BufRead;

#[derive(Debug)]
enum Node {
    Folder(BTreeMap<String, Node>),
    File(usize),
}

impl Node {
    fn run<I: Iterator<Item = String>>(
        &mut self,
        iter: &mut std::iter::Peekable<I>,
    ) -> Result<()> {
        let folders = match self {
            Node::Folder(f) => f,
            Node::File(_) => bail!("Cannot call run on a file"),
        };
        loop {
            let line = match iter.next() {
                Some(line) => line,
                None => return Ok(()),
            };
            let cmd = line
                .strip_prefix("$ ")
                .ok_or_else(|| anyhow!("Missing command in {line}"))?;

            if cmd == "ls" {
                while iter.peek().map(|s| !s.starts_with('$')).unwrap_or(false)
                {
                    let node = iter.next().unwrap();
                    let (name, node) =
                        if let Some(dir) = node.strip_prefix("dir ") {
                            (dir.to_owned(), Node::Folder(Default::default()))
                        } else {
                            let mut iter = node.split(' ');
                            let size = iter
                                .next()
                                .ok_or_else(|| {
                                    anyhow!("Could not read size from {node}")
                                })?
                                .parse()
                                .context("Could not parse file size")?;
                            let file = iter.next().ok_or_else(|| {
                                anyhow!("Could not read filename from {node}")
                            })?;
                            (file.to_owned(), Node::File(size))
                        };
                    if folders.insert(name.clone(), node).is_some() {
                        bail!("Duplicate node {name}");
                    }
                }
            } else if let Some(dir) = cmd.strip_prefix("cd ") {
                if dir == ".." {
                    break;
                } else {
                    folders
                        .get_mut(dir)
                        .ok_or_else(|| anyhow!("Unknown folder {dir}"))?
                        .run(iter)?;
                }
            } else {
                bail!("Unknown command {cmd}");
            }
        }
        Ok(())
    }

    fn folder_sizes(&self, out: &mut Vec<usize>) -> usize {
        match self {
            Node::File(s) => *s,
            Node::Folder(b) => {
                let size = b.values().map(|v| v.folder_sizes(out)).sum();
                out.push(size);
                size
            }
        }
    }
}

fn main() -> Result<()> {
    let mut iter = std::io::stdin()
        .lock()
        .lines()
        .skip(1)
        .map(Result::unwrap)
        .peekable();

    let mut filesystem = Node::Folder(BTreeMap::new());
    filesystem.run(&mut iter)?;

    let mut folder_sizes = vec![];
    let root_size = filesystem.folder_sizes(&mut folder_sizes);

    println!(
        "Part 1: {}",
        folder_sizes
            .iter()
            .filter(|i| **i <= 100_000)
            .sum::<usize>()
    );

    const DISK_SPACE: usize = 70000000;
    const REQUIRED_SPACE: usize = 30000000;
    let unused = DISK_SPACE - root_size;
    let min_free = REQUIRED_SPACE - unused;
    println!(
        "Part 2: {}",
        folder_sizes
            .iter()
            .filter(|i| **i >= min_free)
            .min()
            .ok_or_else(|| anyhow!("No folders meeting min_free"))?
    );
    Ok(())
}
