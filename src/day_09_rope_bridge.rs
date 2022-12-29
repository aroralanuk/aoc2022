use crate::input_reader;
use std::collections::HashSet;

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct GridPos {
    x: isize,
    y: isize,
}

fn part1(input: &String) {
    let mut head = GridPos { x: 0, y: 0 };
    let mut tail = GridPos { x: 0, y: 0 };
    let mut visited: HashSet<GridPos> = HashSet::new();
    visited.insert(tail);

    input.lines().for_each(|line| {
        let (dir, steps) = line.split_once(' ').unwrap();
        let steps = steps.parse::<usize>().unwrap();

        for _ in 0..steps {
            match dir {
                "U" => head.y -= 1,
                "D" => head.y += 1,
                "L" => head.x -= 1,
                "R" => head.x += 1,
                _ => panic!("Unknown direction: {}", dir),
            }

            let distance = GridPos {
                    x: head.x - tail.x,
                    y: head.y - tail.y,
            };

            let touching = distance.x.abs() <= 1 && distance.y.abs() <= 1;

            if !touching {
                tail.x += distance.x.signum();
                tail.y += distance.y.signum();
                visited.insert(tail);
            }
        }
    });

    println!("PART 1 - Visited: {}", visited.len());
}

pub fn main() {
    let input = input_reader::read_file_in_cwd("src/input/day_09.txt");

    part1(&input);
}
