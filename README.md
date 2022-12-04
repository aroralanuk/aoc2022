# Advent of Code 2022

My attempts at the [Advent of Code 2022](https://adventofcode.com/2022) challenges.

## Solutions

* ![Day-01-Calorie-Counting](https://github.com/aroralanuk/aoc2022/actions/workflows/Day-01.yml/badge.svg?branch=main)
* ![Day-01-Rock-Paper-Scissors](https://github.com/aroralanuk/aoc2022/actions/workflows/Day-02.yml/badge.svg?branch=main)

## How to run

All puzzles are compiled into a single executable, and individual puzzles are selected using the first argument, which is the day number (01-25).

For example, to build and execute the puzzle for Day 2, run:

```
cargo run -- 02
```

To build optimized artifacts, run:

```
cargo build –-release --verbose
./target/release/aoc 02
```
