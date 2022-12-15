use crate::input_reader;

fn part1(input: &str) {
    let first_marker = (4..input.len() + 1).find(|&i| unique(&input[i-4..i])).unwrap();

    println!("Part 1: {}", first_marker);
}

fn part2(input: &str) {
    let first_marker = (14..input.len() + 1).find(|&i| unique(&input[i-14..i])).unwrap();

    println!("Part 2: {}", first_marker);
}

fn unique(s: &str) -> bool {
    !s.bytes()
        .enumerate()
        .rev()
        .any(|(i, b)| s[..i].contains(b as char))
}

pub fn main() {
    let input = input_reader::read_file_in_cwd("src/input/day_06.txt");

    part1(&input);
    part2(&input);
}
