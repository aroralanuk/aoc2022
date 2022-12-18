use crate::input_reader;

mod item {
    // treat as a native new type
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    // struct for checking invalid entry
    pub(crate) struct Item(u8);

    // get valid input from file
    impl TryFrom<u8> for Item {
        type Error = color_eyre::Report;

        fn try_from(value: u8) -> Result<Self, Self::Error> {
            match value {
                b'a'..=b'z'| b'A'..=b'Z' => Ok(Item(value)),
                _ => Err(color_eyre::eyre::eyre!(
                    "{} is not a valid item",
                    value as char
                )),
            }
        }
    }

    // debugging format
    impl std::fmt::Debug for Item {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0 as char)
        }
    }

    impl Item {
        pub(crate) fn priority(self) -> usize {
            match self {
                Item(b'a'..=b'z') => 1 + (self.0 - b'a') as usize,
                Item(b'A'..=b'Z') => 27 + (self.0 - b'A') as usize,
                _ => unreachable!(),
            }
        }
    }
}

use item::Item;
use std::collections::HashSet;
use itertools::Itertools;


fn part1() -> color_eyre::Result<()> {
    let mut total = 0;
    let input = input_reader::read_file_in_cwd("src/input/day_03.txt");

    for line in input.lines() {
        // split into the two compartments
        let (first, second) = line.split_at(line.len()/2);

        // get the items from the first compartment
        let first_items = first
            .bytes()
            .map(Item::try_from)
            .collect::<Result<HashSet<_>, _>>()?;

        let dup_second = second
            .bytes()
            .map(Item::try_from)
            // Applies function to the elements of iterator and returns the first non-none result.
            .find_map(|item| {
                item.ok().and_then(|item| {
                    first_items
                        .iter()
                        .copied()
                        .find(|&first_item| first_item == item)
                })
            })
            .expect("there should be exactly one duplicate")
            .priority();
            total += dup_second;

    }

    dbg!(total);

    Ok(())
}

fn part2() -> color_eyre::Result<()> {
    let input = input_reader::read_file_in_cwd("src/input/day_03.txt");
    let rucksacks = input.lines().map(|line | {
        line.bytes().map(Item::try_from)
            .collect::<Result<HashSet<_>, _>>()
    });

    let sum = itertools::process_results(rucksacks, |rs| {
        rs.tuples()
            .map(|(a, b, c)| {
                a.iter()
                    .copied()
                    .find(|i| b.contains(i) && c.contains(i))
                    .map(|i| i.priority())
                    .unwrap_or_default()
            })
            .sum::<usize>()
    })?;
    dbg!(sum);

    Ok(())
}

pub fn main() {
    part1();
    part2();
}
