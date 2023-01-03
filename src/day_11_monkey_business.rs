use crate::input_reader;
use std::str::Lines;

struct Monkey {
    item: Vec<usize>,
    // operation: Operation,
    divisible: usize,
    if_true: usize,
    if_false: usize,
}

fn strip_next<'a>(lines: &mut Lines<'a>, prefix: &str) -> &'a str {
    lines.next().unwrap().strip_prefix(prefix).unwrap()
}

fn parse_items(lines: &mut Lines, prefix: &str) -> Vec<u64> {
    strip_next(lines, prefix)
        .split(", ")
        .map(|item| item.parse().unwrap())
        .collect()
}

fn parse() -> Option<Vec<Monkey>> {
    let input = input_reader::read_file_in_cwd("src/input/day_1.txt");

    let mut monkeys = Vec::new();

    for monke in input.split("\n\n") {
        let mut lines = monke.lines().skip(1);

        let (_, items) = lines.next()?.split_once(": ")?;
        let items = items
            .split(", ")
            .map(|item| item.parse::<usize>().unwrap());

        println!("{:?}", items);
    }

    Some(monkeys)
}
pub fn main() {
    parse();
}

