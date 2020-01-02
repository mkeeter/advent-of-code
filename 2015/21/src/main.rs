use std::cmp::max;
use std::io::BufRead;
use std::str::FromStr;
use itertools::iproduct;

type Item = (i32, i32, i32); // Cost, Damage, Armor

fn main() {
    let input = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();
    let boss_hp = input.iter()
        .filter_map(|d| i32::from_str(&d.replace("Hit Points: ", "")).ok())
        .next()
        .unwrap();
    let boss_damage = input.iter()
        .filter_map(|d| i32::from_str(&d.replace("Damage: ", "")).ok())
        .next()
        .unwrap();
    let boss_armor = input.iter()
        .filter_map(|d| i32::from_str(&d.replace("Armor: ", "")).ok())
        .next()
        .unwrap();

    let weapons = [
        /* Dagger */        (8, 4, 0),
        /* Shortsword */    (10, 5, 0),
        /* Warhammer */     (25, 6, 0),
        /* Longsword */     (40, 7, 0),
        /* Greataxe */      (74, 8, 0),
    ];

    let armors = [
        /* Leather */       (13, 0, 1),
        /* Chainmail */     (31, 0, 2),
        /* Splintmail */    (53, 0, 3),
        /* Bandedmail */    (75, 0, 4),
        /* Platemail */     (102, 0, 5),
        /* No armor */      (0, 0, 0),
    ];

    let rings = [
        /* Damage +1 */     (25, 1, 0),
        /* Damage +2 */     (50, 2, 0),
        /* Damage +3 */     (100, 3, 0),
        /* Defense +1 */    (20, 0, 1),
        /* Defense +2 */    (40, 0, 2),
        /* Defense +3 */    (80, 0, 3),
        /* No ring */       (0, 0, 0),
    ];

    let fight = |items: &[&Item]| -> bool {
        let mut boss_hp = boss_hp;
        let mut your_hp = 100;
        let your_damage: i32 = items.iter().map(|i| i.1).sum();
        let your_armor:  i32 = items.iter().map(|i| i.2).sum();
        loop {
            boss_hp -= max(1, your_damage - boss_armor);
            if boss_hp <= 0 {
                return true;
            }
            your_hp -= max(1, boss_damage - your_armor);
            if your_hp <= 0 {
                return false;
            }
        }
    };

    let win = iproduct!(weapons.iter(), armors.iter(), rings.iter(), rings.iter())
        .filter(|(_, _, ring1, ring2)| ring1 != ring2 || ring1.0 == 0)
        .filter(|(w, a, r, s)| fight(&[w, a, r, s]))
        .map(|(w, a, r, s)| w.0 + a.0 + r.0 + s.0)
        .min()
        .unwrap();
    println!("Part 1: {}", win);

    let lose = iproduct!(weapons.iter(), armors.iter(), rings.iter(), rings.iter())
        .filter(|(_, _, ring1, ring2)| ring1 != ring2 || ring1.0 == 0)
        .filter(|(w, a, r, s)| !fight(&[w, a, r, s]))
        .map(|(w, a, r, s)| w.0 + a.0 + r.0 + s.0)
        .max()
        .unwrap();
    println!("Part 2: {}", lose);
}
