use std::io::Read;
use json::JsonValue;

fn part1(v: &JsonValue) -> i32 {
    match v {
        JsonValue::Number(i) => (*i).into(),
        JsonValue::Object(o) => o.iter().map(|v| part1(v.1)).sum(),
        JsonValue::Array(a)  => a.iter().map(|v| part1(v)).sum(),
        _ => 0,
    }
}

fn part2(v: &JsonValue) -> i32 {
    match v {
        JsonValue::Number(i) => (*i).into(),
        JsonValue::Object(o) =>
            if o.iter().any(|v| v.1 == "red") {
                0
            } else {
                o.iter().map(|v| part2(v.1)).sum()
            },
        JsonValue::Array(a)  => a.iter().map(|v| part2(v)).sum(),
        _ => 0,
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let input = json::parse(&input).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
