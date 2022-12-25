use crate::input_reader;
use std::collections::HashMap;

fn row_col(index: (u32, u32), is_row: bool) -> (u32, u32) {
    if is_row { (index.0, index.1) } else { (index.1, index.0) }
}

fn next_greater(
    grid: &HashMap<(u32, u32), u32>,
    row: u32,
    n: u32,
    inv: bool,
    is_row: bool
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
            res[index.unwrap() as usize] = *num as i32;
        }
        stack.push(i);
        // println!("res {:?}", res);
        // println!("stack {:?}", stack);
        // println!("num {:?}", num);
        }
    res
}

fn part1(grid: &HashMap<(u32, u32), u32>, rows: u32, cols: u32) -> u32
{
    let mut next_greater_right: Vec<Vec<i32>> = vec![];
    let mut next_greater_left: Vec<Vec<i32>> = vec![];
    let mut next_greater_bottom: Vec<Vec<i32>> = vec![];
    let mut next_greater_top: Vec<Vec<i32>> = vec![];

    for i in 0..rows {
        next_greater_right.push(next_greater(grid, i, cols, false, true));
        next_greater_left.push(next_greater(grid, i, cols, true, true));
    }

    for i in 0..cols {
        next_greater_bottom.push(next_greater(grid, i, rows, false, false));
        next_greater_top.push(next_greater(grid, i, rows, true, false));
    }

    // println!("right {:?}", next_greater_right);
    // println!("left {:?}", next_greater_left);
    // println!("bottom {:?}", next_greater_bottom);
    // println!("top {:?}", next_greater_top);

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
    // println!("visible_trees {:?}", visible_trees);
    println!("length {:?}", visible_trees.len());
    5
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
    println!("rows {:?}", row);
    println!("cols {:?}", cols);

    // println!("Part 1: {:?}", grid);
    // println!("Part 2: {:?}", next_greater(&grid, 1, 5, true, true));

    part1(&grid, row, cols);
    // println!("Part 1: {:?}", part1(&grid));
}
