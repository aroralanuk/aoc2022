
// #![warn(clippy::all, clippy::pedantic, unsafe_code)]

use std::fmt::Debug;
use std::str::FromStr;
use std::str::Lines;

use crate::input_reader;

type Operation = Box<dyn Fn(i64) -> i64>;

struct Monkey {
    items: Vec<i64>,
    operation: Operation,
    divisor: i64,
    if_true: usize,
    if_false: usize,
    inspections: usize,
}

fn strip_next<'a>(lines: &mut Lines<'a>, prefix: &str) -> &'a str {
    lines.next().unwrap().strip_prefix(prefix).unwrap()
}

fn parse_next<T>(lines: &mut Lines, prefix: &str) -> T
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    strip_next(lines, prefix).parse().unwrap()
}

fn parse_items(lines: &mut Lines, prefix: &str) -> Vec<i64> {
    strip_next(lines, prefix)
        .split(", ")
        .map(|item| item.parse().unwrap())
        .collect()
}

fn parse_operation(lines: &mut Lines, prefix: &str) -> Operation {
    let operands: Vec<_> = strip_next(lines, prefix).split(' ').collect();

    match operands[..] {
        ["old", "*", "old"] => Box::new(|x| x * x),

        ["old", "*", y] => {
            let y: i64 = y.parse().unwrap();
            Box::new(move |x| x * y)
        }

        ["old", "+", y] => {
            let y: i64 = y.parse().unwrap();
            Box::new(move |x| x + y)
        }

        _ => panic!("{operands:?}"),
    }
}

fn parse() -> Vec<Monkey> {
    let mut monkeys = Vec::new();
    let mut lines = include_str!("input/day_11.txt").lines();
    loop {
        lines.next(); // Skip monkey ID.

        monkeys.push(Monkey {
            items: parse_items(&mut lines, "  Starting items: "),
            operation: parse_operation(&mut lines, "  Operation: new = "),
            divisor: parse_next(&mut lines, "  Test: divisible by "),
            if_true: parse_next(&mut lines, "    If true: throw to monkey "),
            if_false: parse_next(&mut lines, "    If false: throw to monkey "),
            inspections: 0,
        });

        if lines.next().is_none() {
            break;
        }
    }

    monkeys
}

fn simulate<R>(mut monkeys: Vec<Monkey>, rounds: usize, reduction: R) -> usize
where
    R: Fn(i64) -> i64,
{
    for _ in 0..rounds {
        for m in 0..monkeys.len() {
            let monkey = &mut monkeys[m];
            // Apply math to items.
            let items: Vec<_> = monkey
                .items
                .drain(..)
                .map(|item| (monkey.operation)(item))
                .map(&reduction)
                .collect();

            // count inspections currently in hand
            monkey.inspections += items.len();

            // Throw items to other monkes.
            let divisor = monkey.divisor;
            let if_true = monkey.if_true;
            let if_false = monkey.if_false;
            for item in items {
                let target = if item % divisor == 0 { if_true } else { if_false };
                monkeys[target].items.push(item);
            }
        }
    }

    // Extract inspection counts.
    let mut inspections: Vec<_> = monkeys.iter().map(|m| m.inspections).collect();
    // Return product of top two inspection counts.
    inspections.sort_by(|a, b| b.cmp(a));
    inspections.iter().take(2).product()
}

pub fn main() {
    part1();
    part2();
}

fn part1() {
    let monkeys = parse();
    let result = simulate(monkeys, 20, |x| x / 3);

    println!("Part 1: {}", result);
}

fn part2() {
    let monkeys = parse();
    let lcd: i64 = monkeys.iter().map(|m| m.divisor).product();
    let result = simulate(monkeys, 10_000, |x| x % lcd);

    println!("Part 2: {}", result);
}


