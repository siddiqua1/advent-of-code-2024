use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = File::open("inputs/day05.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map_while(Result::ok).collect();

    let mut iter = lines.iter();

    let mut two_way = TwoWayOrdering::default();

    let mut ans = 0;
    while let Some(s) = iter.next() {
        if s.is_empty() {
            break;
        }
        if let Some((a, b)) = s.split_once('|') {
            let x = a.parse().unwrap();
            let y = b.parse().unwrap();
            two_way.add_pair(x, y);
        }
    }

    while let Some(s) = iter.next() {
        'top: {
            let mut seen = HashSet::new();

            let pages: Vec<u32> = s.split(",").map(|x| x.parse().unwrap()).collect();
            for x in &pages {
                if two_way.ordering_wrong(*x, &seen) {
                    break 'top;
                }
                seen.insert(*x);
            }
            ans += pages[pages.len() / 2];
        }
    }

    println!("Part 1 = {ans}");
}

fn part2() {
    let file = File::open("inputs/day05.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map_while(Result::ok).collect();

    let mut iter = lines.iter();

    let mut two_way = TwoWayOrdering::default();

    let mut ans = 0;
    while let Some(s) = iter.next() {
        if s.is_empty() {
            break;
        }
        if let Some((a, b)) = s.split_once('|') {
            let x = a.parse().unwrap();
            let y = b.parse().unwrap();
            two_way.add_pair(x, y);
        }
    }

    while let Some(s) = iter.next() {
        let mut seen = HashSet::new();

        let mut pages: Vec<u32> = s.split(",").map(|x| x.parse().unwrap()).collect();
        let mut should_sort = false;
        for x in &pages {
            if two_way.ordering_wrong(*x, &seen) {
                should_sort = true;
                break;
            }
            seen.insert(*x);
        }
        if should_sort {
            pages.sort_by(|a, b| two_way.get_ord(a, b));
            ans += pages[pages.len() / 2];
        }
    }

    println!("Part 2 = {ans}");
}

#[derive(Default)]
struct TwoWayOrdering {
    less: HashMap<u32, HashSet<u32>>,
}

impl TwoWayOrdering {
    pub fn add_pair(&mut self, a: u32, b: u32) {
        self.less.entry(a).or_default().insert(b);
    }

    pub fn ordering_wrong(&self, a: u32, seen: &HashSet<u32>) -> bool {
        let Some(less) = self.less.get(&a) else {
            return false;
        };
        return less.intersection(seen).count() > 0;
    }

    pub fn get_ord(&self, a: &u32, b: &u32) -> Ordering {
        if let Some(less) = self.less.get(a) {
            if less.contains(b) {
                return Ordering::Less;
            }
        }
        if let Some(more) = self.less.get(b) {
            if more.contains(a) {
                return Ordering::Greater;
            }
        }
        Ordering::Equal
    }
}
