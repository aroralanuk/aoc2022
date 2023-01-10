use crate::input_reader;
use std::cmp::{min, max};
use std::collections::HashSet;

type Map = (usize, usize);

fn part1(cave: &HashSet<Map>, bottom: usize) {
    let mut cave1 = cave.clone();

    let mut path: Vec<Map> = Vec::new();
    path.push((500, 0));

    while let Some(&(x, y)) = path.last() {
        let mut rest = false;
        if y <= bottom {
            for next in [(x + 1, y + 1), (x - 1, y + 1), (x, y + 1)]
                .iter()
            {
                if !cave1.contains(next) {
                    path.push(*next);
                    rest = true;
                }

            }
        }
        if !rest {
            cave1.insert((x, y));
            path.pop();
        } else if y >= bottom {
            break;
        }
    }
    println!("Part1: {}", cave1.len() - cave.len());
}

fn part2(cave: &HashSet<Map>, bottom: usize) {
    let mut cave1 = cave.clone();

    let mut path: Vec<Map> = Vec::new();
    path.push((500, 0));

    while let Some(&(x, y)) = path.last() {
        let mut rest = false;
        if y <= bottom {
            for next in [(x + 1, y + 1), (x - 1, y + 1), (x, y + 1)]
                .iter()
            {
                if !cave1.contains(next) {
                    path.push(*next);
                    rest = true;
                }

            }
        }
        if !rest {
            cave1.insert((x, y));
            path.pop();
        }
    }
    println!("Part2: {}", cave1.len() - cave.len());
}

pub fn main() {
    let input = input_reader::read_file_in_cwd("src/input/day_14.txt");

    let mut cave: HashSet<Map> = HashSet::new();
    let mut bottom = 0;
    input.lines().for_each(|line| {
        line.split(" -> ").fold(None, |coord: Option<Map>, coord_str| {
            let (a, b): Map = coord_str
                .split_once(',')
                .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
                .unwrap();
            bottom = max(bottom, b);
            if let Some((a0, b0)) = coord {
                if a == a0 {
                    for i in min(b, b0)..=max(b, b0) {
                        cave.insert((a, i));
                    }
                } else if b == b0 {
                    for i in min(a, a0)..=max(a, a0) {
                        cave.insert((i, b));
                    }
                }
            } else {
                cave.insert((a, b));
            }
            Some((a, b))
        });

        part1(&cave, bottom);
        part2(&cave, bottom);
    });
}
