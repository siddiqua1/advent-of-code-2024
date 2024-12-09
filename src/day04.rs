use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = File::open("inputs/day04.txt").unwrap();
    let reader = BufReader::new(file);

    let mut ans = 0;
    let chars: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line.chars().collect())
        .collect();
    let n = chars.len();
    let m = chars[0].len();

    for i in 0..n {
        for j in 0..m {
            if j + 3 < m {
                // left
                // covers right
                ans += is_xmas(
                    chars[i][j],
                    chars[i][j + 1],
                    chars[i][j + 2],
                    chars[i][j + 3],
                ) as u32;
            }
            if i + 3 < n {
                // down
                // covers up
                ans += is_xmas(
                    chars[i][j],
                    chars[i + 1][j],
                    chars[i + 2][j],
                    chars[i + 3][j],
                ) as u32;
            }

            if j + 3 < m && i + 3 < n {
                // diagonal down and left
                // covers diagonal up and right
                ans += is_xmas(
                    chars[i][j],
                    chars[i + 1][j + 1],
                    chars[i + 2][j + 2],
                    chars[i + 3][j + 3],
                ) as u32;
            }
            if j >= 3 && i + 3 < n {
                // diagonal down and right
                // covers diagonal up and left
                ans += is_xmas(
                    chars[i][j],
                    chars[i + 1][j - 1],
                    chars[i + 2][j - 2],
                    chars[i + 3][j - 3],
                ) as u32;
            }
        }
    }

    println!("Part 1 = {ans}");
}

fn part2() {
    let file = File::open("inputs/day04.txt").unwrap();
    let reader = BufReader::new(file);

    let mut ans = 0;
    let chars: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line.chars().collect())
        .collect();
    let n = chars.len();
    let m = chars[0].len();

    for i in 1..(n - 1) {
        for j in 1..(m - 1) {
            if chars[i][j] != 'A' {
                continue;
            }
            // assuming that no other 4 letter combination of X,M,A,S sums
            // to the same value
            const SSMM: u32 = 2 * ('S' as u32) + 2 * ('M' as u32);
            let sum = (chars[i - 1][j - 1] as u32)
                + (chars[i - 1][j + 1] as u32)
                + (chars[i + 1][j - 1] as u32)
                + (chars[i + 1][j + 1] as u32);
            if sum == SSMM && chars[i - 1][j - 1] != chars[i + 1][j + 1] {
                ans += 1;
            }
        }
    }

    println!("Part 2 = {ans}");
}

fn is_xmas(a: char, b: char, c: char, d: char) -> bool {
    const XMAS: u32 = 1000 * ('X' as u32) + 100 * ('M' as u32) + 10 * ('A' as u32) + ('S' as u32);
    const SAMX: u32 = ('X' as u32) + 10 * ('M' as u32) + 100 * ('A' as u32) + 1000 * ('S' as u32);

    let sum = 1000 * (a as u32) + 100 * (b as u32) + 10 * (c as u32) + (d as u32);
    sum == XMAS || sum == SAMX
}
