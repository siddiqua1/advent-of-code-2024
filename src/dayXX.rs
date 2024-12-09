use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = File::open("inputs/day03.txt").unwrap();
    let reader = BufReader::new(file);

    let ans = reader
        .lines()
        .map(|line| line.unwrap())
        .fold(0, |acc, _line| acc + 1);

    println!("Part 1 = {ans}");
}

fn part2() {
    let file = File::open("inputs/day03.txt").unwrap();
    let reader = BufReader::new(file);

    let ans = reader
        .lines()
        .map(|line| line.unwrap())
        .fold(0, |acc, _line| acc + 1);

    println!("Part 2 = {ans}");
}
