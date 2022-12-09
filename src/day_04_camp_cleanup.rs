use crate::input_reader;

fn part1(sections: &Vec<((i32,i32), (i32, i32))>) {
    let mut full_overlap = 0;
    for sec in sections {
        if (sec.0.0 <= sec.1.0 && sec.0.1 >= sec.1.1) ||
        (sec.0.0 >= sec.1.0 && sec.0.1 <= sec.1.1) {
            full_overlap += 1
        }
    }

    println!("Part 1: {}", full_overlap);
}

fn part2(sections: &Vec<((i32,i32), (i32, i32))>) {
    let mut partial_overlap = 0;
    for sec in sections {
        if (sec.0.0 <= sec.1.0 && sec.0.1 >= sec.1.0)
        || (sec.1.0 <= sec.0.0 && sec.1.1 >= sec.0.0) {
            partial_overlap += 1
        }
    }

    println!("Part 2: {}", partial_overlap);
}

pub fn main() {
    let input = input_reader::read_file_in_cwd("src/input/day_04.txt");

    let lines: Vec<&str> = input.split("\n").collect();
    let sections: Vec<((i32,i32), (i32, i32))> = lines.iter().map(|&val| {
        let pairs: Vec<&str> = val.split(",").collect();
        let first_pair: Vec<&str> = pairs[0].split("-").collect();
        let second_pair: Vec<&str> = pairs[1].split("-").collect();
        return (
            (first_pair[0].parse::<i32>().unwrap(), first_pair[1].parse::<i32>().unwrap()),
            (second_pair[0].parse::<i32>().unwrap(), second_pair[1].parse::<i32>().unwrap())
        )
    }).collect();

    part1(&sections);
    part2(&sections);
}
