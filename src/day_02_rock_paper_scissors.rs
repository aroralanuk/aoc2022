
use crate::input_reader;

#[derive(Copy, Clone, PartialEq, Eq)]
enum Play {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

fn part1(games: &Vec<(Play, Play)>) {
    let mut score = 0;
    for g in games {
        match g {
            (opp,res) if opp == res => score += 3 + *res as i32,
            (Play::Rock, Play::Paper) => score += 8,
            (Play::Paper, Play::Scissors) => score += 9,
            (Play::Scissors, Play::Rock) => score += 7,
            (_, res) => score += *res as i32,
        }
    }
    println!("Part 1: {}", score);
}

fn part2(games: &Vec<(Play, Play)>) {
    let mut score = 0;
    for g in games {
        match g {
            (opp,Play::Paper) => score += 3 + *opp as i32,
            (opp, Play::Scissors) => score += 7 + (*opp as i32 % 3),
            (Play::Rock, _) => score += 3,
            (opp, _) => score += (*opp as i32+ 2) % 3,
        }
    }
    println!("Part 2: {}", score);
}

pub fn main() {
    let input = input_reader::read_file_in_cwd("src/day_02_input.txt");

    let lines: Vec<&str> = input.split("\n").collect();
    let games: Vec<(Play, Play)> = lines.iter().map(|&val| {
        let line_split: Vec<&str> = val.split(" ").collect();
        let opp: Play = match line_split[0] {
            "A" => Play::Rock,
            "B" => Play::Paper,
            "C" => Play::Scissors,
            _ => panic!("Invalid input."),
        };
        let res: Play = match line_split[1] {
            "X" => Play::Rock,
            "Y" => Play::Paper,
            "Z" => Play::Scissors,
            _ => panic!("Invalid input."),
        };
        return (opp, res)
    }).collect();

    part1(&games);
    part2(&games);
}
