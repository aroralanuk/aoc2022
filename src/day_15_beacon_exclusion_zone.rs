use crate::input_reader;
use itertools::Itertools;

fn part1(beacons: &[(isize, isize, isize)]) {
    let check_at_200000 = beacons.iter()
        .map(|&(x,y,d) | (x, d - (2000000 - y).abs()))
        .filter(|&(_,trace)| trace >= 0)
        .flat_map(|(x,trace)| [(x-trace, true), (x + trace + 1, false)])
        .sorted()
        .collect::<Vec<_>>();

    let (mut ans, mut inside) = (-1,1);
    for ((prev, _), &(x, start)) in check_at_200000.iter().tuple_windows() {
        // println!("{:?} {:?} {:?}", prev, x, start);
        if inside > 0 {ans += x - prev }
        inside += if start { 1 } else { -1 };
    }

    println!("{:?}", ans);
}

fn part2(beacons: &[(isize, isize, isize)]) {
    let mut ans = 0;
    for &(x,y,d) in beacons {
        for (dx, dy) in [(-1,-1), (-1,1), (1,-1), (1,1)] {
            for dist in 0..d {
                let bx = x + dx * dist;
                let by = y + dy * (d + 1 - dist);
                if bx < 0 || by < 0 || bx > 4000000 || by > 4000000 { break; }
                if beacons.iter().all(|&(x,y,d)| (x-bx).abs() + (y-by).abs() >= d) {
                    ans = bx * 4000000 + by;
                }
            }
        }
    }

    println!("{:?}", ans);
}


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

    part1(&beacons);
    part2(&beacons);
}
