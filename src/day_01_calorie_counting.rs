use crate::input_reader;

fn part1(elfs_calories: &Vec<Vec<i32>>) {
    // Elf with the most calories.
    let mut max_total_calories = 0;
    for e in 0..elfs_calories.len() {
        let mut total_calories_for_elf = 0;
        for c in 0..elfs_calories[e].len() {
            total_calories_for_elf += elfs_calories[e][c];
        }
        if max_total_calories < total_calories_for_elf {
            max_total_calories = total_calories_for_elf;
        }
    }
    println!("Part 1: {}", max_total_calories);
}

fn part2(elfs_calories: &mut Vec<Vec<i32>>) {
    let mut sum_top_3 = 0;
    let mut max_total_calories: [i32; 3] = [0, 0, 0];
    let mut elf_index = 0;
    for i in 0..3 {
        for e in 0..elfs_calories.len() {
            let mut total_calories_for_elf = 0;
            for c in 0..elfs_calories[e].len() {
                total_calories_for_elf += elfs_calories[e][c];
            }
            if max_total_calories[i] < total_calories_for_elf {
                max_total_calories[i] = total_calories_for_elf;
                elf_index = e;
            }
        }
        sum_top_3 += max_total_calories[i];
        elfs_calories.remove(elf_index);
    }

    println!("Part 2: {}", sum_top_3);
}

pub fn main() {
    let input = input_reader::read_file_in_cwd("src/day_01_input.txt");

    // Parse the calorie counts.
    let elfs_calories_str: Vec<&str> = input.split("\n\n").collect();
    let mut elfs_calories: Vec<Vec<i32>> = elfs_calories_str.iter().map(|&val| {
        let elf_calories_str: Vec<&str> = val.split("\n").collect();
        let elf_calories: Vec<i32> = elf_calories_str.iter().map(|&val| {
            return val.parse::<i32>().unwrap();
        }).collect();
        return elf_calories;
    }).collect();

    part1(&elfs_calories);
    part2(&mut elfs_calories);
}
