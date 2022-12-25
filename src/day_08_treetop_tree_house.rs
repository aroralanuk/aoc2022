use crate::input_reader;
use std::collections::HashMap;
use std::cmp;

fn row_col(index: (u32, u32), is_row: bool) -> (u32, u32) {
    if is_row { (index.0, index.1) } else { (index.1, index.0) }
}

fn next_greater(
    grid: &HashMap<(u32, u32), u32>,
    row: u32,
    n: u32,
    inv: bool,
    is_row: bool,
    scenic_score: bool
) -> Vec<i32> {
    let mut res: Vec<i32>= vec![-1; n as usize];
    let mut stack: Vec<u32> = vec![];

    for k in 0..n {
        let i = if inv { n - 1 - k } else { k };
        let ind_i = row_col((row, i), is_row);
        let num = grid.get(&ind_i).unwrap();

        while !stack.is_empty() && num >= grid.get(
            &row_col((row, stack[stack.len() - 1]), is_row)).unwrap() {
            let index = stack.pop();
            res[index.unwrap() as usize] =
                if scenic_score {i as i32}
                 else {*num as i32};
        }
        stack.push(i);
        }
    res
}

fn part1(grid: &HashMap<(u32, u32), u32>, rows: u32, cols: u32)
{
    let mut next_greater_right: Vec<Vec<i32>> = vec![];
    let mut next_greater_left: Vec<Vec<i32>> = vec![];
    let mut next_greater_bottom: Vec<Vec<i32>> = vec![];
    let mut next_greater_top: Vec<Vec<i32>> = vec![];

    for i in 0..rows {
        next_greater_right.push(next_greater(grid, i, cols, false, true, false));
        next_greater_left.push(next_greater(grid, i, cols, true, true, false));
    }

    for i in 0..cols {
        next_greater_bottom.push(next_greater(grid, i, rows, false, false, false));
        next_greater_top.push(next_greater(grid, i, rows, true, false, false));
    }

    let mut visible_trees: Vec<u32> = vec![];

    for i in 0..rows {
        for j in 0..cols {
            if next_greater_right[i as usize][j as usize] == -1 ||
                next_greater_left[i as usize][j as usize] == -1 ||
                next_greater_bottom[j as usize][i as usize] == -1 ||
                next_greater_top[j as usize][i as usize] == -1
            {
                visible_trees.push(*grid.get(&(i, j)).unwrap());
            }
        }
    }
    println!("Visible trees {:?}", visible_trees.len());
}

fn part2(grid: &HashMap<(u32, u32), u32>, rows: u32, cols: u32)
{
    let mut next_greater_right: Vec<Vec<i32>> = vec![];
    let mut next_greater_left: Vec<Vec<i32>> = vec![];
    let mut next_greater_bottom: Vec<Vec<i32>> = vec![];
    let mut next_greater_top: Vec<Vec<i32>> = vec![];

    for i in 0..rows {
        next_greater_right.push(next_greater(grid, i, cols, false, true, true));
        next_greater_left.push(next_greater(grid, i, cols, true, true, true));
    }

    for i in 0..cols {
        next_greater_bottom.push(next_greater(grid, i, rows, false, false, true));
        next_greater_top.push(next_greater(grid, i, rows, true, false, true));
    }

    let mut highest_scenic_score: u32 = 0;
    for i in 0..rows {
        for j in 0..cols {
            let right_index = next_greater_right[i as usize][j as usize];
            let right_dist =
                if right_index > 0 { right_index as u32 - j }
                else {cols - j - 1};

            let left_index = next_greater_left[i as usize][j as usize];
            let left_dist =
                if left_index > 0 { j - left_index as u32 }
                else {j};

            let top_index = next_greater_top[j as usize][i as usize];
            let top_dist =
                if top_index > 0 { i - (top_index as u32) }
                else {i};

            let bottom_index = next_greater_bottom[j as usize][i as usize];
            let bottom_dist =
                if bottom_index > 0 { bottom_index as u32 - i }
                else {rows - i - 1};

            highest_scenic_score =  cmp::max(
                highest_scenic_score,
                right_dist * left_dist * top_dist * bottom_dist);
        }
    }
    println!("Highest scenic score {:?}", highest_scenic_score);

}

pub fn main() {
    let input = input_reader::read_file_in_cwd("src/input/day_08.txt");

    let mut grid: HashMap<(u32, u32), u32> = HashMap::new();
    let mut row: u32 = 0;
    let mut col: u32 = 0;
    let mut cols: u32 = 0;
    input.lines().for_each(|line| {
        line.chars().for_each(|x| {
            grid.insert((row, col), x.to_digit(10).unwrap());
            col += 1;
        });
        row += 1;
        cols = col;
        col = 0;
    });
    println!("rows x cols = {:?} x {:?}", row, cols);

    part1(&grid, row, cols);
    part2(&grid, row, cols);
}
