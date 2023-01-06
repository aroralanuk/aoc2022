use crate::input_reader;
use std::collections::VecDeque;

fn bfs(map: &[Vec<u8>], start: &[(usize, usize)], end: (usize, usize)) -> Option<usize> {
  let mut visited = vec![vec![false; map[0].len()]; map.len()];
  let mut queue = start.iter().map(|&(x,y)| (x, y, 0)).collect::<VecDeque<_>>();
  while let Some((x, y, len)) = queue.pop_front() {
    if (x, y) == end {
      return Some(len);
    }
    for (dx, dy) in [(0,-1), (-1,0), (0,1), (1,0)] {
      let (nx, ny) = ((x as isize + dx) as usize, (y as isize + dy) as usize);
      let Some(&square) = map.get(nx).and_then(|row| row.get(ny)) else { continue };
      if map[x][y] + 1 >= square && !visited[nx][ny] {
        visited[nx][ny] = true;
        queue.push_back((nx, ny, len + 1));
      }
    }
  }
  None
}


pub fn main() {
    let input = input_reader::read_file_in_cwd("src/input/day_12.txt");
    let mut map = input.lines().map(|l| l.as_bytes().to_vec()).collect::<Vec<_>>();

    let mut start = (0,0);
    let mut end = (0,0);
    let mut all_starts = vec![];

    for x in 0..map.len() {
        for y in 0..map[0].len() {
            if map[x][y] == b'S' {
                start = (x,y);
                map[x][y] = b'a';
            }
            if map[x][y] == b'E' {
                end = (x,y);
                map[x][y] = b'z';
            }
            if map[x][y] == b'a' {
                all_starts.push((x,y));
            }
        }
    }

    println!("Part1: {}", bfs(&map, &[(start.0, start.1)], (end.0, end.1)).unwrap());
    println!("Part2: {}", bfs(&map, &all_starts, (end.0, end.1)).unwrap());

}
