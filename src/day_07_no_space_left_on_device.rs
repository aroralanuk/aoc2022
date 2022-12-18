use crate::input_reader;
use std::collections::HashMap;
use regex::Regex;

fn part1(sizes: &HashMap<String, u32>) {
    let total: u32 = sizes.values().filter( |&size| *size < 100_000).sum();

    println!("Total size is {}", total);

}

fn part2(sizes: &HashMap<String, u32>) {
   let space_needed: u32 = sizes.get("<root>").unwrap() - 40_000_000;

    let smallest_dir = sizes.iter()
    .filter(|(_dir, &size)| size >= space_needed)
    .min_by_key(|(_dir, &size)| size)
    .unwrap();
    println!("Smallest dir is {:?}", smallest_dir);
}


pub fn main() {
    let input = input_reader::read_file_in_cwd("src/input/day_07.txt");
    let mut path:Vec<&str> = vec![];
    let mut sizes:HashMap<String, u32> = HashMap::new();
    let cd_regex = Regex::new(r"^\$ cd (\S+)").unwrap();
    let file_regex = Regex::new(r"^(\d+) (\S+)").unwrap();

    for line in input.lines() {
        if cd_regex.is_match(line) {
            let caps = cd_regex.captures(line).unwrap();
            let mut dir = caps.get(1).unwrap().as_str();
            if dir == ".." {
                path.pop();
            } else {
                if dir == "/" {
                    dir = "<root>";
                }
                path.push(dir);
                let path_str = path.join("/");
                if ! sizes.contains_key(&path_str) {
                    sizes.insert(path_str, 0);
                }
            }
        } else if file_regex.is_match(line) {
            let caps = file_regex.captures(line).unwrap();
            let file: u32 = caps.get(1).unwrap().as_str().parse().unwrap();
            let mut update_list: Vec<&str> = vec![];
            for p in &path {
                update_list.push(*p);
                let update_path = update_list.join("/");
                sizes.entry(update_path).and_modify(|e| *e += file);
            }
        }
        // println!("{:?}",sizes);
    }

    part1(&sizes);
    part2(&sizes);
}
