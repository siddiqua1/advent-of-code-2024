use std::{collections::HashMap, fs};

fn main() {
    part1();
    part2();
}

fn part1() {
    // read in the file contents
    let contents =
        fs::read_to_string("inputs/day01-p1.txt").expect("Should have been able to read the file");

    // initialize the left and right lists
    // note we can preallocate the size based on the number of lines
    let capacity = contents.lines().count();
    let mut left: Vec<i64> = Vec::with_capacity(capacity);
    let mut right: Vec<i64> = Vec::with_capacity(capacity);

    // iterate contents line by line collecting into each list
    for line in contents.lines() {
        if let Some((a, b)) = line.split_once("   ") {
            let l = a
                .parse::<i64>()
                .expect("unable to parse first part to an integer");
            left.push(l);
            let r = b
                .parse::<i64>()
                .expect("unable to parse second part to an integer");
            right.push(r);
        }
    }
    left.sort();
    right.sort();

    let ans = left
        .iter()
        .zip(right.iter())
        .fold(0, |acc, (a, b)| acc + i64::abs(a - b));

    println!("Part 1 = {ans}")
}

fn part2() {
    // read in the file contents (same input)
    let contents =
        fs::read_to_string("inputs/day01-p1.txt").expect("Should have been able to read the file");

    // initialize the left list as a vector
    // the right list is a frequency map
    let capacity = contents.lines().count();
    let mut left: Vec<i64> = Vec::with_capacity(capacity);
    let mut right: HashMap<i64, i64> = HashMap::with_capacity(capacity);

    // iterate contents line by line collecting the left into a list
    // and updating the right's frequency in the map
    for line in contents.lines() {
        if let Some((a, b)) = line.split_once("   ") {
            let l = a
                .parse::<i64>()
                .expect("unable to parse first part to an integer");
            left.push(l);

            let r = b
                .parse::<i64>()
                .expect("unable to parse second part to an integer");
            *right.entry(r).or_insert(0) += 1;
        }
    }

    let ans = left.iter().fold(0, |acc, &x| {
        let k = *right.get(&x).unwrap_or(&0);
        acc + x * k
    });

    println!("Part 2 = {ans}")
}
