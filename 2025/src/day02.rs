pub fn solve(s: &str) -> (i64, i64) {
    println!("{s}");
    for pair in s.split(',') {
        let mut iter = pair.split('-');
        let a = iter.next().unwrap();
        let b = iter.next().unwrap();
        assert!(iter.next().is_none());
        println!("{a} {b}");
        let a: usize = a.parse().unwrap();
        let b: usize = b.parse().unwrap();
    }
    let _ = s;
    unimplemented!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let s = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let (a, b) = solve(s);
        assert_eq!(a, 1227775554);
    }
}
