use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = File::open("inputs/day02-p1.txt").unwrap();
    let reader = BufReader::new(file);

    let ans = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line_to_levels(&line))
        .fold(0, |acc, level| acc + level_safety(&level));

    println!("Part 1 = {ans}");
}

fn line_to_levels(line: &str) -> Vec<i64> {
    line.split(" ").map(|part| part.parse().unwrap()).collect()
}

fn level_safety(level: &[i64]) -> i64 {
    let sign = if level[0] < level[1] { 1 } else { -1 };

    for pair in level.windows(2) {
        if !pair_is_safe(pair[0], pair[1], sign) {
            return 0;
        }
    }
    1
}

fn part2() {
    let file = File::open("inputs/day02-p1.txt").unwrap();
    let reader = BufReader::new(file);

    let ans = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line_to_levels(&line))
        .fold(0, |acc, level| acc + level_safety_tolerance(&level));

    println!("Part 2 = {ans}");
}

fn pair_is_safe(x: i64, y: i64, sign: i64) -> bool {
    let diff = y - x;
    let abs = diff * sign;
    abs > 0 && abs < 4
}

fn calculate_sign_of_level(level: &[i64]) -> Option<i64> {
    let mut increment = 0;
    let mut decrement = 0;
    // for every pair see if it increments or not
    for pair in level.windows(2) {
        if pair[0] < pair[1] {
            increment += 1;
        } else {
            decrement += 1;
        }
    }
    // at the end return the bigger one as long as the smaller one is less than 3
    if increment > decrement {
        if decrement < 3 {
            return Some(1);
        }
        return None;
    }
    if increment < 3 {
        return Some(-1);
    }
    None
}

fn level_safety_tolerance(level: &[i64]) -> i64 {
    let Some(sign) = calculate_sign_of_level(level) else {
        return 0;
    };
    let mut removed_level: Option<usize> = None;

    for i in 0..(level.len() - 2) {
        // we have (a, b, c)
        let (a, b, c) = (level[i], level[i + 1], level[i + 2]);

        // check if this range is safe
        if pair_is_safe(a, b, sign) && pair_is_safe(b, c, sign) {
            continue;
        }
        // if it isn't then need to check if any sub pair is safe
        // if we can skip first one
        if pair_is_safe(b, c, sign) {
            if let Some(x) = removed_level {
                if x != i {
                    return 0;
                }
            }
            removed_level = Some(i);
            continue;
        }
        // can skip the second value
        if pair_is_safe(a, c, sign) {
            if let Some(x) = removed_level {
                if x != i + 1 {
                    return 0;
                }
            }
            removed_level = Some(i + 1);
            continue;
        }
        // can skip third
        if pair_is_safe(a, b, sign) {
            if let Some(x) = removed_level {
                if x != i + 2 {
                    return 0;
                }
            }
            removed_level = Some(i + 2);
            continue;
        }
        // no subpair is safe
        return 0;
    }
    1
}
