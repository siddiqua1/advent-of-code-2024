use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = "inputs/day12.txt";
    let plots = Garden::from(file);
    let (ans, _) = plots.fencing_price();
    println!("Part 1 = {ans}");
}

fn part2() {
    let file = "inputs/day12.txt";
    let plots = Garden::from(file);
    let (_, ans) = plots.fencing_price();

    println!("Part 2 = {ans}");
}

struct Garden {
    grid: Vec<Vec<char>>,
    n: usize,
    m: usize,
}

impl Garden {
    pub fn from(file: &str) -> Self {
        let file = File::open(file).unwrap();
        let reader = BufReader::new(file);

        let grid: Vec<Vec<char>> = reader
            .lines()
            .map(|line| line.unwrap())
            .map(|line| line.chars().collect())
            .collect();
        let n = grid.len();
        let m = grid[0].len();

        Self { grid, n, m }
    }

    pub fn fencing_price(&self) -> (u64, u64) {
        let mut explored: HashSet<(usize, usize)> = HashSet::new();
        let mut ans1 = 0;
        let mut ans2 = 0;
        for i in 0..self.n {
            for j in 0..self.m {
                if explored.contains(&(i, j)) {
                    continue;
                }
                let (area, perimeter, sides) = self.explore(i, j, &mut explored);

                ans1 += area * perimeter;
                ans2 += area * sides;
            }
        }
        (ans1, ans2)
    }

    fn up_is_same(&self, i: usize, j: usize, group: char) -> bool {
        i > 0 && group == self.grid[i - 1][j]
    }
    fn down_is_same(&self, i: usize, j: usize, group: char) -> bool {
        i + 1 < self.n && group == self.grid[i + 1][j]
    }
    fn right_is_same(&self, i: usize, j: usize, group: char) -> bool {
        j + 1 < self.m && group == self.grid[i][j + 1]
    }
    fn left_is_same(&self, i: usize, j: usize, group: char) -> bool {
        j > 0 && group == self.grid[i][j - 1]
    }

    fn up_left_is_same(&self, i: usize, j: usize, group: char) -> bool {
        i > 0 && j > 0 && group == self.grid[i - 1][j - 1]
    }
    fn up_right_is_same(&self, i: usize, j: usize, group: char) -> bool {
        i > 0 && j + 1 < self.m && group == self.grid[i - 1][j + 1]
    }
    fn down_left_is_same(&self, i: usize, j: usize, group: char) -> bool {
        i + 1 < self.n && j > 0 && group == self.grid[i + 1][j - 1]
    }
    fn down_right_is_same(&self, i: usize, j: usize, group: char) -> bool {
        i + 1 < self.n && j + 1 < self.m && group == self.grid[i + 1][j + 1]
    }

    fn explore(
        &self,
        x: usize,
        y: usize,
        explored: &mut HashSet<(usize, usize)>,
    ) -> (u64, u64, u64) {
        let mut queue = VecDeque::new();
        let mut area = 0;
        let mut perimeter = 0;
        let mut corners = 0;
        let group = self.grid[x][y];
        queue.push_back((x, y));

        while let Some((i, j)) = queue.pop_front() {
            if explored.contains(&(i, j)) {
                continue;
            }
            area += 1;
            corners += self.corner_count(i, j, group);
            explored.insert((i, j));

            if self.up_is_same(i, j, group) {
                queue.push_back((i - 1, j));
            } else {
                perimeter += 1;
            }

            if self.down_is_same(i, j, group) {
                queue.push_back((i + 1, j));
            } else {
                perimeter += 1;
            }

            if self.left_is_same(i, j, group) {
                queue.push_back((i, j - 1));
            } else {
                perimeter += 1;
            }

            if self.right_is_same(i, j, group) {
                queue.push_back((i, j + 1));
            } else {
                perimeter += 1;
            }
        }

        (area, perimeter, corners)
    }

    fn is_corner(&self, diagonal: bool, vertical: bool, horizontal: bool) -> bool {
        match (diagonal, vertical, horizontal) {
            (false, a, b) if a == b => true,
            (false, a, b) if a != b => false,
            (true, false, false) => true,
            (_, _, _) => false,
        }
    }

    fn corner_count(&self, i: usize, j: usize, group: char) -> u64 {
        let mut ans = 0;

        if self.is_corner(
            self.up_left_is_same(i, j, group),
            self.up_is_same(i, j, group),
            self.left_is_same(i, j, group),
        ) {
            ans += 1;
        }

        if self.is_corner(
            self.up_right_is_same(i, j, group),
            self.up_is_same(i, j, group),
            self.right_is_same(i, j, group),
        ) {
            ans += 1;
        }

        if self.is_corner(
            self.down_left_is_same(i, j, group),
            self.down_is_same(i, j, group),
            self.left_is_same(i, j, group),
        ) {
            ans += 1;
        }

        if self.is_corner(
            self.down_right_is_same(i, j, group),
            self.down_is_same(i, j, group),
            self.right_is_same(i, j, group),
        ) {
            ans += 1;
        }

        ans
    }
}
