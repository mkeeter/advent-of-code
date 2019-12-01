fn part1() {
    let mut recipes = vec![3, 7];
    let mut a = 0;
    let mut b = 1;

    const N: usize = 939601;

    for _i in 0..=(N + 10) {
        // Add new recipes to the list
        let sum = recipes[a] + recipes[b];
        if sum >= 10 {
            recipes.push(sum / 10);
        }
        recipes.push(sum % 10);

        a = (a + recipes[a] + 1) % recipes.len();
        b = (b + recipes[b] + 1) % recipes.len();
    }

    let sol = recipes[N..N + 10].iter().map(|i| i.to_string()).collect::<String>();
    println!("Part 1: {}", sol);
}

fn part2() {
    let mut recipes = vec![3, 7];
    let mut a = 0;
    let mut b = 1;

    let target = "939601"
            .chars()
            .map(|d| d.to_digit(10).unwrap())
            .map(|i| i as usize)
            .collect::<Vec<usize>>();

    let check_tail = |recipes: &Vec<usize>| -> bool {
        recipes.len() >= target.len() &&
        recipes[recipes.len() - target.len()..].iter().eq(target.iter())
    };

    loop {
        // Add new recipes to the list
        let sum = recipes[a] + recipes[b];
        if sum >= 10 {
            recipes.push(sum / 10);
            if check_tail(&recipes) {
                break;
            }
        }
        recipes.push(sum % 10);
        if check_tail(&recipes) {
            break;
        }

        a = (a + recipes[a] + 1) % recipes.len();
        b = (b + recipes[b] + 1) % recipes.len();
    }
    println!("Part 2: {}", recipes.len() - target.len());
}

fn main() {
    part1();
    part2();
}
