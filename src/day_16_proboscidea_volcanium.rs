use crate::input_reader;
use std::{fs, collections::HashMap, usize};

#[derive(Debug)]
struct RawValve {
    label: String,
    index: usize,
    flow: u32,
    neighbors: Vec<String>,
}

#[derive(Debug)]
struct Valve {
    label: String,
    flow: u32,
    neighbors: Vec<usize>,
}



fn parse(input: &str) -> Vec<Valve> {
    let mut index = 0;
    let mut indices: HashMap<String, usize> = HashMap::new();
    let raw: Vec<RawValve> = input.lines().map(|line| {
        let (valve, neighbors) = line.split_once("; ").unwrap();
        let valve = valve.strip_prefix("Valve ").unwrap();
        let (label, flow) = valve.split_once(" has flow rate=").unwrap();
        let flow = flow.parse().unwrap();
        let neighbors = neighbors
            .strip_prefix("tunnels lead to valves ")
            .or_else(|| neighbors.strip_prefix("tunnel leads to valve "))
            .unwrap();
        let neighbors = neighbors.split_terminator(", ").map(|s| s.to_string()).collect();

        indices.insert(label.to_string(), index);
        index += 1;

        RawValve {
            label: label.to_string(),
            index: index - 1,
            flow,
            neighbors,
        }
    }).collect();

    raw.into_iter()
        .map(|raw| {

            Valve {
                label: raw.label,
                flow: raw.flow,
                neighbors: raw.neighbors.into_iter().map(|n| indices[&n]).collect(),
            }
        }).collect()
}

fn matrix(list: &Vec<Valve>) -> Vec<Vec<u32>> {
    let l = list.len();
    let mut graph = vec![vec![u32::MAX / 4; l]; l];

    list
        .iter()
        .enumerate()
        .for_each(|(i, val)| {
            val.neighbors
                .iter()
                .for_each(|j| {
                    graph[i][*j] = 1;
                });
        });

    graph
}

fn floyd_warshall(matrix: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let l = matrix.len();
    let mut min_distances = matrix.clone();

    for k in 0..l {
        for i in 0..l {
            for j in 0..l {
                if min_distances[i][k] + min_distances[k][j] < min_distances[i][j] {
                    min_distances[i][j] = min_distances[i][k] + min_distances[k][j];
                }
            }
        }
    }
    min_distances
}

pub fn main() {
    let input = input_reader::read_file_in_cwd("src/input/day_16.txt");

    let valves = parse(&input);
    println!("{:?}", valves);

    let graph = matrix(&valves);
    println!("{:?}", graph);

    let dist = floyd_warshall(graph);
    println!("{:?}", dist);

    let start_idx = valves.iter().position(|x| x.label == "AA").unwrap();
    let len = dist.len();
}
