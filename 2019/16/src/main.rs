use std::io::Read;
use std::collections::HashMap;

type Cache = HashMap<(i32, usize, usize), i32>;
fn _dft<'a>(level: i32, skip: usize, step: usize, input: &Vec<i32>,
            cache: &mut Cache) -> i32
{
    let key = (level, skip, step);
    if let Some(i) = cache.get(&key) {
        return *i;
    }

    if level == 0 {
        0
    }
    else if level == 1 {
        let out = [1, 0, -1, 0].iter()
            .cycle()
            .zip(input.iter().skip(skip).step_by(step))
            .map(|(a, b)| a * b )
            .sum::<i32>().abs() % 10;
        cache.insert(key, out);
        out
    } else {
        let a = _dft(level - 1, skip*2 + 2, step + 1, input, cache);
        let b = _dft(level - 1, skip*2 + 1, step + 1, input, cache);
        let out = (a + b) % 10;
        cache.insert(key, out);
        out
    }
}

fn dft<'a>(level: i32, input: &Vec<i32>, cache: &mut Cache) -> i32 {
    _dft(level, 0, 1, input, cache)
}

fn main() {
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s).unwrap();

    let mut input = s.chars()
        .filter_map(|c| char::to_digit(c, 10))
        .map(|i| i as i32)
        .collect::<Vec<i32>>();

    let mut c = Cache::new();
    println!("{}", dft(1, &input, &mut c));
    println!("{}", dft(2, &input, &mut c));
    println!("{}", dft(3, &input, &mut c));
    println!("{}", dft(4, &input, &mut c));
    println!("{}", dft(5, &input, &mut c));
    println!("{}", dft(6, &input, &mut c));
    println!("{}", dft(7, &input, &mut c));
    println!("{}", dft(8, &input, &mut c));
    println!("{:?}", c);

    for _ in 0..1 {
        let mut c = Cache::new();
        input = (0..input.len())
            .map(|i| dft(i as i32, &input, &mut c))
            .collect::<Vec<i32>>();
    }

    /*
    print!("Part 1: ");
    for c in input[..8].iter() {
        print!("{}", c);
    }
    println!("");

    let size = input.len();
    let mut input = s.chars()
        .filter_map(|c| char::to_digit(c, 10))
        .map(|i| i as i32)
        .cycle()
        .take(size * 10000)
        .collect::<Vec<i32>>();
    let offset = input[..7].iter().fold(0, |acc, i| acc * 10 + i);

    println!("{}", size);
    for x in 0..100 {
        let mut h = HashSet::new();
        let mut next = Vec::new();
        for i in 0..input.len() {
            let pattern = [0, 1, 0, -1].iter()
                .flat_map(|j| std::iter::once(j).cycle().take(i + 1))
                .cycle()
                .skip(1)
                .collect::Vec<i32>();
        }

        println!("{}", x);
        input = (0..input.len()).map(|i|
            [0, 1, 0, -1].iter()
                .flat_map(|j| std::iter::once(j).cycle().take(i + 1))
                .cycle()
                .skip(1)
                .zip(input.iter())
                .map(|(a, b)| a * b )
                .sum::<i32>().abs() % 10
        ).collect::<Vec<i32>>();
    }
    */
}
