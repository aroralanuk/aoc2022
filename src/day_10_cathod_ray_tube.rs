use crate::input_reader;

fn part1(input: &String) {
    let mut cycles = 1;
    let mut register = 1;
    let mut sig_strength = 0;

    input.lines().for_each(|line| {
        sig_strength += (cycles % 40 == 20) as i32 * cycles * register;

        cycles += 1;

        let mut split = line.split_whitespace();

        match split.next().unwrap() {
            "addx" => {
                sig_strength += (cycles % 40 == 20) as i32 * cycles * register;
                register += split.next().unwrap().parse::<i32>().unwrap();
                cycles += 1;
            }
            "noop" => {},
            _ => panic!(),
        };

    });
    println!("addx {}", sig_strength);
}


fn get_pixel(cycle: usize, x: i32) -> char {
    let curr_col = (cycle - 1) % 40;
    if (curr_col as i32).abs_diff(x) <=  1 {
        '#'
    } else {
        '.'
    }
}

fn part2(input: &String) {
    let mut cycles = 1;
    let mut register = 1;
    let mut crt = [' '; 40 * 6];

    input.lines().for_each(|line| {
        crt[cycles - 1] = get_pixel(cycles, register);

        cycles += 1;

        let mut split = line.split_whitespace();

        match split.next().unwrap() {
            "addx" => {
                crt[cycles - 1] = get_pixel(cycles, register);
                register += split.next().unwrap().parse::<i32>().unwrap();
                cycles += 1;
            }
            "noop" => {},
            _ => panic!(),
        };

    });
    let image = crt
        .chunks(40)
        .map(|row| row.iter().collect())
        .collect::<Vec<String>>()
        .join("\n");

    println!("Part2: ");
    println!("{}", image);
}


pub fn main() {
    let input = input_reader::read_file_in_cwd("src/input/day_10.txt");

    part1(&input);
    part2(&input);
}
