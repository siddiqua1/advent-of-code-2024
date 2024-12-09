use core::fmt;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = File::open("inputs/day09.txt").unwrap();
    let reader = BufReader::new(file);

    let ans = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| DiskMap::from(&line))
        .map(|disk| disk.compress())
        .fold(0, |acc, disk| acc + disk.checksum());

    println!("Part 1 = {ans}");
}

fn part2() {
    let file = File::open("inputs/day09.txt").unwrap();
    let reader = BufReader::new(file);

    let ans = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| DiskMap::from(&line))
        .map(|disk| disk.compress_no_fragmentation())
        .fold(0, |acc, disk| {
            // println!("{disk}");
            acc + disk.checksum()
        });

    println!("Part 2 = {ans}");
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct FileBlock {
    // if None this block is free
    id: Option<u32>,
    size: u32,
}

impl FileBlock {
    pub fn expand(&self) -> Vec<Self> {
        let mut res = Vec::with_capacity(self.size as usize);
        for _ in 0..self.size {
            res.push(FileBlock {
                id: self.id,
                size: 1,
            });
        }
        res
    }

    pub fn is_free(&self) -> bool {
        self.id.is_none()
    }
}

struct DiskMap {
    files: Vec<FileBlock>,
}

impl DiskMap {
    pub fn from(line: &str) -> Self {
        let mut files = Vec::new();
        let mut curr_id = 0;
        let mut is_free = false;

        line.chars()
            .map(|c| c.to_digit(10).unwrap())
            .for_each(|digit| {
                files.push(FileBlock {
                    id: if is_free { None } else { Some(curr_id) },
                    size: digit,
                });
                if !is_free {
                    curr_id += 1
                }
                is_free = !is_free;
            });

        DiskMap { files }
    }

    pub fn compress(&self) -> Self {
        let mut files: Vec<FileBlock> =
            self.files.iter().flat_map(|block| block.expand()).collect();

        let mut left = 0;
        let mut right = files.len() - 1;
        while left < right {
            while !files[left].is_free() {
                left += 1;
            }
            while files[right].is_free() {
                right -= 1;
            }
            if left >= right {
                break;
            }

            (files[left], files[right]) = (files[right], files[left]);

            left += 1;
            right -= 1;
        }

        DiskMap { files }
    }

    pub fn checksum(&self) -> u64 {
        let files: Vec<FileBlock> = self.files.iter().flat_map(|block| block.expand()).collect();
        files.iter().enumerate().fold(0, |acc, (i, block)| {
            let Some(id) = block.id else {
                return acc;
            };
            acc + (id as u64) * (i as u64)
        })
    }

    fn free_block_with_min_size(size: u32, list: &[FileBlock]) -> Option<usize> {
        for (i, blk) in list.iter().enumerate() {
            if blk.is_free() && blk.size >= size {
                return Some(i);
            }
        }
        None
    }

    pub fn compress_no_fragmentation(&self) -> Self {
        let mut files: Vec<Vec<FileBlock>> = self.files.iter().map(|block| vec![*block]).collect();

        let mut right = files.len() - 1;
        while right > 0 {
            if files[right][0].is_free() {
                right = right.saturating_sub(1);
                continue;
            }
            // find left most free space
            let space = files[right][0].size;
            let mut left = 0;

            while left < right {
                let Some(i) = Self::free_block_with_min_size(space, &files[left]) else {
                    left += 1;
                    continue;
                };

                let extra = files[left][i].size - space;
                files[left][i].id = files[right][0].id;
                files[left][i].size = space;
                files[right][0].id = None;

                files[left].push(FileBlock {
                    id: None,
                    size: extra,
                });
                break;
            }
            right = right.saturating_sub(1);
        }

        DiskMap {
            files: files.into_iter().flatten().collect::<Vec<FileBlock>>(),
        }
    }
}

impl fmt::Display for DiskMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for block in &self.files {
            let writing = if block.is_free() {
                ".".to_owned()
            } else {
                block.id.unwrap().to_string()
            };
            for _ in 0..block.size {
                write!(f, "{writing} | ")?;
            }
        }
        Ok(())
    }
}
