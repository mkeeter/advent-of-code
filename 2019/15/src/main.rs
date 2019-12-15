use std::io::Read;
use std::collections::HashMap;
use vm::Vm;

struct Robot {
    pos: (i64, i64),
    history: Vec<i64>,
    vm: Vm,
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let vm = Vm::from_str(&input);

    let mut todo = Vec::new();
    todo.push(Robot { pos: (0, 0), history: vec![], vm: vm });

    let mut seen = HashMap::new();

    while let Some(bot) = todo.pop()
    {
        for (cmd, delta) in [(1, ( 0,  1)),
                             (2, ( 0, -1)),
                             (3, (-1,  0)),
                             (4, ( 1,  0))].iter()
        {
            let pos = (bot.pos.0 + delta.0, bot.pos.1 + delta.1);
            if !seen.contains_key(&pos) {
                let mut next = Robot {
                    pos: pos,
                    history: bot.history.clone(),
                    vm: bot.vm.clone(),
                };
                next.history.push(*cmd);
                next.vm.input(*cmd);
                if let Some(o) = next.vm.run_until() {
                    seen.insert(pos, o);
                    match o {
                        0 => (),
                        1 => todo.push(next),
                        2 => println!("Part 1: {}", next.history.len()),
                        _ => panic!("Invalid output"),
                    };
                }
            }
        }
    }
}
