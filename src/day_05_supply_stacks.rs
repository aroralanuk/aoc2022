use crate::input_reader;
use regex::Regex;

struct Instruction {
    crates: i32,
    from: usize,
    to: usize
}

fn part1(stacks: &mut Vec<Vec<char>>, instructions: &Vec<Instruction>) {
    for ins in instructions {
        for _ in 0..ins.crates {
            let krate = stacks[ins.from].pop().unwrap();
            stacks[ins.to].push(krate);
        }
    }
    let mut soln = "".to_owned();
    for stack in stacks {
        soln.push(*stack.last().unwrap());
    }
    println!("Part 1: {}", soln);
}

fn part2(stacks: &mut Vec<Vec<char>>, instructions: &Vec<Instruction>) {
    for ins in instructions {
        let mut krate_grp: Vec<char> = vec![];
        for _ in 0..ins.crates {
            krate_grp.insert(0, stacks[ins.from].pop().unwrap());
        }
        stacks[ins.to].append(&mut krate_grp);
    }
    let mut soln = "".to_owned();
    for stack in stacks {
        soln.push(*stack.last().unwrap());
    }
    println!("Part 2: {}", soln);
}

pub fn main() {
    let input = input_reader::read_file_in_cwd("src/input/day_05.txt");
    let input_slices: Vec<&str> = input.split("\n\n").collect();

    let positions: Vec<&str> = input_slices[0].split("\n").collect();
    let stack_height = positions.len() - 1;

    let stacks_count = (positions.last().unwrap().len() + 1) / 4; // 3 spaces in between
    let mut stacks: Vec<Vec<char>> = vec![vec![]; stacks_count];
    let stack_regex = Regex::new(r"\[[A-Z]\]|    ").unwrap();
    let crate_regex = Regex::new(r"[A-Z]").unwrap();
    for row_index in (0..stack_height).rev() {
        let stack_item = positions[row_index];
        let mut col_index = 0;
        for item_match in stack_regex.captures_iter(stack_item) {
            // checks for blank spaces
            if item_match[0].starts_with(" ") {
                col_index += 1;
                continue;
            }
            let item = &crate_regex.captures(&item_match[0]).unwrap()[0];
            stacks[col_index].push(item.chars().next().unwrap());
            col_index += 1;
        }
    }

    let instruction_str: Vec<&str> = input_slices[1].split("\n").collect();
    let instructions: Vec<Instruction> = instruction_str.iter().map(|&val| {
        let split: Vec<&str> = val.split(" ").collect();
        let crates = split[1].parse::<i32>().unwrap();
        let from = split[3].parse::<usize>().unwrap() - 1;
        let to = split[5].parse::<usize>().unwrap() - 1;
        return Instruction { crates, from, to };
    }).collect();

    part1(&mut stacks.clone(), &instructions);
    part2(&mut stacks, &instructions);

}
