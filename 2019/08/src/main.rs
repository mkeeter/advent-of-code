use std::io::Read;

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).unwrap();

    let data: Vec<u32> = buffer
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<_>>();

    let width = 25;
    let height = 6;
    let layer = data
        .chunks(width * height)
        .min_by_key(|layer| layer.iter().filter(|i| **i == 0).count())
        .unwrap();
    let num_ones = layer.iter().filter(|i| **i == 1).count();
    let num_twos = layer.iter().filter(|i| **i == 2).count();
    println!("Part 1: {}", num_ones * num_twos);

    println!("Part 2:");
    let combined = data
        .chunks(width * height)
        .fold(vec![2; width * height], |combined, layer| {
            combined
                .iter()
                .zip(layer.iter())
                .map(|(p, q)| if *p == 2 { *q } else { *p })
                .collect()
        });

    let image = combined
        .iter()
        .map(|i| if *i == 1 { 'X' } else { ' ' })
        .collect::<Vec<_>>();

    for row in image.chunks(width) {
        println!("{}", row.iter().collect::<String>());
    }
}
