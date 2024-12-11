use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = "inputs/day10.txt";
    let map = TopographicalMap::from(file);
    let (ans, _) = map.trailhead_scores();
    println!("Part 1 = {ans}");
}

fn part2() {
    let file = "inputs/day10.txt";
    let map = TopographicalMap::from(file);
    let (_, ans) = map.trailhead_scores();

    println!("Part 2 = {ans}");
}

#[derive(Debug)]
struct TopographicalMap {
    trailheads: Vec<(usize, usize)>,
    heights: Vec<Vec<u32>>,
    n: usize,
    m: usize,
}

impl TopographicalMap {
    pub fn from(file: &str) -> Self {
        let mut trailheads = Vec::new();
        let mut heights = Vec::new();

        let file = File::open(file).unwrap();
        let reader = BufReader::new(file);
        let lines = reader.lines();

        for (i, line) in lines.enumerate() {
            let Ok(l) = line else {
                continue;
            };
            heights.push(Vec::new());
            for (j, c) in l.chars().enumerate() {
                let d = c.to_digit(10).unwrap();
                if d == 0 {
                    trailheads.push((i, j));
                }
                heights[i].push(d);
            }
        }
        let n = heights.len();
        let m = heights[0].len();

        Self {
            trailheads,
            heights,
            n,
            m,
        }
    }

    pub fn bfs(&self, trailhead: (usize, usize)) -> (u32, u32) {
        let mut bfs = VecDeque::new();
        bfs.push_back(trailhead);

        let mut peaks = HashSet::new();
        let mut trails = 0;
        while !bfs.is_empty() {
            let Some((x, y)) = bfs.pop_front() else {
                panic!("should not get None from an non-empty deque");
            };
            let curr = self.heights[x][y];
            if curr == 9 {
                peaks.insert((x, y));
                trails += 1;
                continue;
            }

            if x > 0 && curr + 1 == self.heights[x - 1][y] {
                bfs.push_back((x - 1, y));
            }
            if x + 1 < self.n && curr + 1 == self.heights[x + 1][y] {
                bfs.push_back((x + 1, y));
            }
            if y > 0 && curr + 1 == self.heights[x][y - 1] {
                bfs.push_back((x, y - 1));
            }
            if y + 1 < self.m && curr + 1 == self.heights[x][y + 1] {
                bfs.push_back((x, y + 1));
            }
        }
        (peaks.len() as u32, trails)
    }

    pub fn trailhead_scores(&self) -> (u32, u32) {
        self.trailheads.iter().fold((0, 0), |acc, th| {
            let (peaks, trails) = self.bfs(*th);
            (acc.0 + peaks, acc.1 + trails)
        })
    }
}
