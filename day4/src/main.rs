use std::str::FromStr;

#[macro_use] extern crate nom;
use nom::types::CompleteByteSlice;

#[macro_use] extern crate itertools;

use std::collections::HashMap;

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Event {
    year: usize,
    month: usize,
    day: usize,
    hour: usize,
    minute: usize,
    action: Action,
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Action {
    Begin(usize),
    Sleep,
    Wake,
}

named!(usize_<CompleteByteSlice, usize>,
       map_res!(recognize!(nom::digit), |s:CompleteByteSlice|
                usize::from_str(std::str::from_utf8(s.0).unwrap())));

named!(action<CompleteByteSlice, Action>,
       alt!(value!(Action::Sleep, tag!("falls asleep")) |
            value!(Action::Wake,  tag!("wakes up")) |
            do_parse!(tag!("Guard #") >> id: usize_ >> tag!(" begins shift")
                                      >> (Action::Begin(id)))));

named!(parse_line<CompleteByteSlice, Event>,
    do_parse!(tag!("[") >>
              year: usize_ >>
              tag!("-") >>
              month: usize_ >>
              tag!("-") >>
              day: usize_ >>
              tag!(" ") >>
              hour: usize_ >>
              tag!(":") >>
              minute: usize_ >>
              tag!("] ") >> action: action >>
              (Event { year: year, month: month, day: day,
                       hour: hour, minute: minute, action: action})));

named!(parse_lines<CompleteByteSlice, Vec<Event>>,
       many0!(do_parse!( p: parse_line >> tag!("\n") >> (p))));

fn main() {
    let input = include_bytes!("../input");
    let mut events: Vec<Event> = parse_lines(CompleteByteSlice(input)).unwrap().1;
    events.sort();

    let mut sleeping = HashMap::new();
    events.iter().fold((0, 0), |(id, sleep), e| {
        match e.action {
            Action::Begin(id) => (id, 0),
            Action::Sleep => (id, e.minute),
            Action::Wake => {
                let minutes = sleeping.entry(id).or_insert([0; 60]);
                for i in sleep..e.minute {
                    minutes[i] += 1;
                };
                (id, 0)
            },
        }
    });

    let (guard, _) = sleeping.iter()
        .map(|(g, mins)| (g, mins.iter().sum::<usize>()))
        .max_by_key(|k| k.1).unwrap();
    let min = sleeping[guard].iter().enumerate().max_by_key(|k| k.1).unwrap().0;
    println!("Part 1: {}", *guard * min);

    let (min, (guard, _)) = iproduct!(0..60, sleeping.iter())
        .max_by_key(|k| (k.1).1[k.0]).unwrap();
    println!("Part 2: {}", min * guard);
}
