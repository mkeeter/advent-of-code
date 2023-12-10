use std::collections::BTreeMap;

#[derive(Debug)]
struct RangeMap(BTreeMap<usize, usize>);

impl RangeMap {
    fn new() -> Self {
        let mut out = BTreeMap::new();
        out.insert(0, 0);
        Self(out)
    }
    fn insert(&mut self, dest: usize, src: usize, n: usize) {
        self.0.insert(src, dest);
        use std::collections::btree_map::Entry;
        if let Entry::Vacant(e) = self.0.entry(src + n) {
            e.insert(src + n);
        }
    }
    fn get(&self, i: usize) -> usize {
        let k = self.0.range(..=i).last().unwrap();
        k.1 + i - k.0
    }

    fn get_range(
        &self,
        mut i: std::ops::Range<usize>,
    ) -> Vec<std::ops::Range<usize>> {
        // Artificially split the input range based on map split points
        let mut ranges = vec![];
        for (&k, _) in self.0.range(i.clone()) {
            ranges.push(i.start..k);
            i = k..i.end
        }
        ranges.push(i);
        ranges
            .into_iter()
            .filter(|r| !r.is_empty())
            .map(|r| {
                let start = self.get(r.start);
                start..start + r.len()
            })
            .collect()
    }
}

pub fn solve(s: &str) -> (String, String) {
    let lines = s.lines().collect::<Vec<&str>>();

    let seeds = lines[0]
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut map = RangeMap::new();
    let mut maps = vec![];
    for line in lines[3..].iter().filter(|line| !line.is_empty()) {
        if line.contains("map") {
            maps.push(map);
            map = RangeMap::new();
        } else {
            let mut iter =
                line.split_whitespace().map(|s| s.parse::<usize>().unwrap());
            let dest = iter.next().unwrap();
            let source = iter.next().unwrap();
            let n = iter.next().unwrap();
            map.insert(dest, source, n);
        }
    }
    maps.push(map);

    let mut out = seeds.clone();
    for m in &maps {
        out.iter_mut().for_each(|s| *s = m.get(*s));
    }
    let p1 = out.iter().min().unwrap();

    let mut out = seeds
        .chunks(2)
        .map(|s| s[0]..s[0] + s[1])
        .collect::<Vec<_>>();
    for m in &maps {
        out = out.into_iter().flat_map(|s| m.get_range(s)).collect();
    }
    let p2 = out.iter().map(|s| s.start).min().unwrap();

    (p1.to_string(), p2.to_string())
}
