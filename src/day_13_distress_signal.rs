use crate::input_reader;
use std::cmp::Ordering;

fn compare(left: &[u8], right: &[u8]) -> Ordering {
    match (left[0],right[0]) {
        (a,b) if a==b => compare(&left[1..], &right[1..]),
        (b']', _) => Ordering::Less,
        (_, b']') => Ordering::Greater,
        (_, b'[') => {
            let l = [&[left[0], b']'],&left[1..]].concat();
            compare(&l, &right[1..])
        },
        (b'[', _) => {
            let r = [&[right[0], b']'],&right[1..]].concat();
            compare(&left[1..], &r)
        },
        (_,_) => left[0].cmp(&right[0])
    }
}


pub fn part1(input: &str) {
    let result: i64 = input.split("\n\n")
    .enumerate()
    .filter(|(_,s)|{
        let (left, right) = s.split_once('\n').unwrap();
        // println!("s: {:?}", s);
        compare(left.as_bytes(), right.as_bytes()) == Ordering::Less
    })
    .map(|(i,_)| i as i64 + 1 )
    .sum();

    println!("Part1: {}", result);
}


pub fn part2(input: &str) {
    let mut all_signals = input.replace("\n\n", "\n");

    all_signals.push_str("\n[[2]]");
    all_signals.push_str("\n[[6]]");

    let mut sorted = all_signals.lines().collect::<Vec<&str>>();
    sorted.sort_by(|left,right| compare(left.as_bytes(), right.as_bytes()));

    // 1 indexed
    let mut decoder_keys = (0,0);

    for ( i, sig) in sorted.iter().enumerate() {
        if sig.eq(&"[[2]]") {
            decoder_keys.0 = i + 1;
        } else if decoder_keys.0 != 0 && sig.eq(&"[[6]]") {
            decoder_keys.1 = i + 1;
        }
    }
    println!("Part2: {}", decoder_keys.0*decoder_keys.1);
}

pub fn main() {
    let input = input_reader::read_file_in_cwd("src/input/day_13.txt");
    let input = input.replace("10", "A");

    part1(&input);
    part2(&input);
}
