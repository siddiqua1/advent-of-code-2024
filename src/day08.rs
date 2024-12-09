use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = File::open("inputs/day08.txt").unwrap();
    let reader = BufReader::new(file);

    let chars: Vec<_> = reader
        .lines()
        .map(|line| line.unwrap().chars().collect::<Vec<char>>())
        .collect();
    let mapping = FrequencyMap::from(&chars);
    let ans = mapping.get_antinode_count();

    println!("Part 1 = {ans}");
}

fn part2() {
    let file = File::open("inputs/day08.txt").unwrap();
    let reader = BufReader::new(file);

    let chars: Vec<_> = reader
        .lines()
        .map(|line| line.unwrap().chars().collect::<Vec<char>>())
        .collect();
    let mapping = FrequencyMap::from(&chars);
    let ans = mapping.get_resonant_antinode_count();

    println!("Part 2 = {ans}");
}

struct FrequencyMap {
    freq: HashMap<char, Vec<(i32, i32)>>,
    n: usize,
    m: usize,
}

impl FrequencyMap {
    pub fn from(grid: &[Vec<char>]) -> Self {
        let mut freq: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

        let n = grid.len();
        let m = grid[0].len();

        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == '.' {
                    continue;
                }
                freq.entry(grid[i][j])
                    .or_default()
                    .push((i as i32, j as i32));
            }
        }

        Self { freq, n, m }
    }
    pub fn in_bounds(&self, loc: (i32, i32)) -> bool {
        let (i, j) = loc;
        i >= 0 && j >= 0 && i < self.n as i32 && j < self.m as i32
    }

    pub fn get_antinode_locs(a: (i32, i32), b: (i32, i32)) -> ((i32, i32), (i32, i32)) {
        let (a1, a2) = a;
        let (b1, b2) = b;
        ((2 * b1 - a1, 2 * b2 - a2), (2 * a1 - b1, 2 * a2 - b2))
    }

    pub fn get_antinode_count(&self) -> u32 {
        let mut seen: HashSet<_> = HashSet::new();
        self.freq.values().fold(0, |acc, list| {
            let l = list.len();
            let mut count = 0;
            for i in 0..l {
                for j in (i + 1)..l {
                    let (l1, l2) = Self::get_antinode_locs(list[i], list[j]);

                    if self.in_bounds(l1) && !seen.contains(&l1) {
                        count += 1;
                    }
                    if self.in_bounds(l2) && !seen.contains(&l2) {
                        count += 1;
                    }

                    seen.insert(l1);
                    seen.insert(l2);
                }
            }

            acc + count
        })
    }

    pub fn get_resonant_antinode_locs(&self, a: (i32, i32), b: (i32, i32)) -> Vec<(i32, i32)> {
        let (a1, a2) = a;
        let (b1, b2) = b;
        let (d1, d2) = (b1 - a1, b2 - a2);
        let mut locs: Vec<(i32, i32)> = Vec::new();

        let mut adding = (a1 + d1, a2 + d2);
        while self.in_bounds(adding) {
            locs.push(adding);
            adding = (adding.0 + d1, adding.1 + d2);
        }
        // need to include current node position as well
        let mut subtracting = (a1, a2);
        while self.in_bounds(subtracting) {
            locs.push(subtracting);
            subtracting = (subtracting.0 - d1, subtracting.1 - d2);
        }

        locs
    }

    pub fn get_resonant_antinode_count(&self) -> u32 {
        let mut seen: HashSet<_> = HashSet::new();
        let ans = self.freq.values().fold(0, |acc, list| {
            let l = list.len();
            let mut count = 0;
            for i in 0..l {
                for j in (i + 1)..l {
                    let locs = self.get_resonant_antinode_locs(list[i], list[j]);

                    for loc in locs {
                        if !seen.contains(&loc) {
                            count += 1;
                        }
                        seen.insert(loc);
                    }
                }
            }

            acc + count
        });

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_anti_locs() {
        let a = (4, 6);
        let b = (5, 4);
        let anti1 = (6, 2);
        let anti2 = (3, 8);

        let (l1, l2) = FrequencyMap::get_antinode_locs(a, b);

        assert!((l1 == anti1 && l2 == anti2) || (l2 == anti1 && l1 == anti2));
    }
}
