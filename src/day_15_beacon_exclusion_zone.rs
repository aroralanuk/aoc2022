use crate::input_reader;
use itertools::Itertools;


pub fn main() {
    let input = input_reader::read_file_in_cwd("src/input/day_15.txt");

    let beacons = input.lines().map(|l|
        l.split(|ch: char| !ch.is_digit(10) && ch != '-')
        .filter_map(|s| s.parse::<isize>().ok())
        .collect_tuple()
        .map(|(x,y,dx,dy)| (x, y, (x-dx).abs() + (y-dy).abs()))
        .unwrap()
    ).collect::<Vec<_>>();

    println!("{:?}", beacons);
}
