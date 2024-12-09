use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = File::open("inputs/day06.txt").unwrap();
    let reader = BufReader::new(file);

    let mut guard = Guard::default();
    let grid: Vec<Vec<Grid>> = reader
        .lines()
        .map(|line| line.unwrap())
        .enumerate()
        .map(|(x, line)| {
            line.chars()
                .enumerate()
                .map(|(y, c)| match c {
                    '^' => {
                        guard.set(x as i32, y as i32);
                        Grid::Clear
                    }
                    '.' => Grid::Clear,
                    '#' => Grid::Blocked,
                    _ => panic!("Unexpected character in input"),
                })
                .collect()
        })
        .collect();

    let positions = get_travelled_positions(guard, &grid);

    let ans = positions.iter().fold(0, |acc, (_pos, set)| {
        acc + if set.is_empty() { 0 } else { 1 }
    });

    println!("Part 1 = {ans}");
}

fn part2() {
    let file = File::open("inputs/day06.txt").unwrap();
    let reader = BufReader::new(file);

    let mut guard = Guard::default();
    let mut grid: Vec<Vec<Grid>> = reader
        .lines()
        .map(|line| line.unwrap())
        .enumerate()
        .map(|(x, line)| {
            line.chars()
                .enumerate()
                .map(|(y, c)| match c {
                    '^' => {
                        guard.set(x as i32, y as i32);
                        Grid::Clear
                    }
                    '.' => Grid::Clear,
                    '#' => Grid::Blocked,
                    _ => panic!("Unexpected character in input"),
                })
                .collect()
        })
        .collect();

    // try every positions :)
    let mut ans = 0;
    let n = grid.len();
    let m = grid[0].len();

    for i in 0..n {
        for j in 0..m {
            if let Grid::Blocked = grid[i][j] {
                continue;
            }

            // block the position
            grid[i][j] = Grid::Blocked;
            // check if we ever loop
            if would_loop(guard, &grid) {
                ans += 1;
            }
            // unblock
            grid[i][j] = Grid::Clear;
        }
    }

    println!("Part 2 = {ans}");
}

fn get_travelled_positions(
    mut guard: Guard,
    grid: &[Vec<Grid>],
) -> HashMap<(i32, i32), HashSet<Direction>> {
    let mut positions: HashMap<(i32, i32), HashSet<Direction>> = HashMap::new();

    positions
        .entry(guard.position())
        .or_default()
        .insert(guard.direction);

    while guard.step(grid) {
        positions
            .entry(guard.position())
            .or_default()
            .insert(guard.direction);
    }

    positions
}

fn would_loop(mut guard: Guard, grid: &[Vec<Grid>]) -> bool {
    let mut visited: HashMap<(i32, i32), HashSet<Direction>> = HashMap::new();
    while guard.step(grid) {
        if let Some(set) = visited.get(&guard.position()) {
            if set.contains(&guard.direction) {
                return true;
            }
        }
        visited
            .entry(guard.position())
            .or_default()
            .insert(guard.direction);
    }

    false
}

#[allow(unused)]
fn print_completed(grid: &[Vec<Grid>], visited: &HashSet<(i32, i32)>) {
    let n = grid.len();
    let m = grid[0].len();

    for i in 0..n {
        for j in 0..m {
            if visited.contains(&(i as i32, j as i32)) {
                print!("X");
                continue;
            }
            match grid[i][j] {
                Grid::Clear => print!("."),
                Grid::Blocked => print!("#"),
            }
        }
        println!();
    }
}

#[derive(Debug)]
enum Grid {
    Blocked,
    Clear,
}

#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn forward(&self, x: i32, y: i32) -> (i32, i32) {
        match self {
            Direction::Up => (x - 1, y),
            Direction::Right => (x, y + 1),
            Direction::Down => (x + 1, y),
            Direction::Left => (x, y - 1),
        }
    }
    pub fn turn(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
struct Guard {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Guard {
    pub fn set(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    pub fn step(&mut self, grid: &[Vec<Grid>]) -> bool {
        let n = grid.len() as i32;
        let m = grid[0].len() as i32;

        let (x, y) = self.direction.forward(self.x, self.y);

        if x < 0 || x >= n || y < 0 || y >= m {
            return false;
        }

        if let Grid::Blocked = grid[x as usize][y as usize] {
            self.direction = self.direction.turn();
        } else {
            self.x = x;
            self.y = y;
        }
        true
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}
