use rayon::prelude::*;
use std::collections::HashMap;
use std::fs;

fn main() {
    part1();
    part2();
}

fn part1() {
    let stones: String = fs::read_to_string("inputs/day11.txt").unwrap();
    let stones = StoneLine::from(&stones);

    let ans = (0..25).fold(stones, |acc, _| acc.blink()).count();
    println!("Part 1 = {ans}");
}

fn part2() {
    let stones: String = fs::read_to_string("inputs/day11.txt").unwrap();
    let stones = StoneLine::from(&stones);
    let ans = stones.better_blink(75);
    println!("Part 2 = {ans}");
}

pub fn log_10(num: u64) -> u64 {
    let mut pow = 1;
    let mut curr = 10;

    while curr <= num {
        curr *= 10;
        pow += 1
    }
    pow
}

fn split_if_even(num: u64) -> Option<(u64, u64)> {
    let digits = log_10(num);

    if digits % 2 == 1 {
        return None;
    }
    let divider = 10_u64.pow((digits / 2) as u32);
    let left = num / (divider);
    let right = num % (divider);

    Some((left, right))
}

pub fn stone_morph(stone: u64) -> Vec<u64> {
    if stone == 0 {
        return vec![1];
    }
    if let Some((a, b)) = split_if_even(stone) {
        return vec![a, b];
    }
    vec![2024 * stone]
}

#[derive(Debug, PartialEq, Eq)]
struct StoneLine {
    stones: Vec<u64>,
}

impl StoneLine {
    pub fn from(line: &str) -> Self {
        let parts = line.split(" ");
        let stones = parts.into_iter().map(|s| s.parse().unwrap()).collect();
        Self { stones }
    }

    pub fn blink(&self) -> Self {
        let stones = self
            .stones
            .par_iter()
            .flat_map(|stone| stone_morph(*stone))
            .collect();

        Self { stones }
    }

    pub fn count(&self) -> u64 {
        self.stones.len() as u64
    }

    pub fn amortized_blink(
        stone: u64,
        iterations: u64,
        amortization: &mut HashMap<u64, HashMap<u64, u64>>,
    ) -> u64 {
        // amortized[stone] = iterations -> count
        if iterations == 0 {
            return 1;
        }

        if let Some(map) = amortization.get(&stone) {
            if let Some(amortized_count) = map.get(&iterations) {
                return *amortized_count;
            }
        }

        if stone == 0 {
            let count = Self::amortized_blink(1, iterations - 1, amortization);

            let prev = amortization
                .entry(stone)
                .or_default()
                .insert(iterations, count);

            if let Some(count_prime) = prev {
                if count != count_prime {
                    panic!("Should not be possible to blink a stone to different counts given same iteration amount")
                }
            }

            return count;
        }

        if let Some((left, right)) = split_if_even(stone) {
            let left_count = Self::amortized_blink(left, iterations - 1, amortization);

            let right_count = Self::amortized_blink(right, iterations - 1, amortization);

            let prev = amortization
                .entry(stone)
                .or_default()
                .insert(iterations, left_count + right_count);

            if let Some(count_prime) = prev {
                if left_count + right_count != count_prime {
                    panic!("Should not be possible to blink a stone to different counts given same iteration amount")
                }
            }

            return left_count + right_count;
        }

        let count = Self::amortized_blink(stone * 2024, iterations - 1, amortization);

        let prev = amortization
            .entry(stone)
            .or_default()
            .insert(iterations, count);

        if let Some(count_prime) = prev {
            if count != count_prime {
                panic!("Should not be possible to blink a stone to different counts given same iteration amount")
            }
        }
        count
    }

    pub fn better_blink(&self, iterations: u64) -> u64 {
        let mut amortization: HashMap<u64, HashMap<u64, u64>> = HashMap::new();

        let mut ans = 0;
        for stone in &self.stones {
            ans += Self::amortized_blink(*stone, iterations, &mut amortization);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digit_count_test() {
        let x = 1;
        assert_eq!(log_10(x), 1);

        let x = 10;
        assert_eq!(log_10(x), 2);

        let x = 100;
        assert_eq!(log_10(x), 3);

        let x = 1_000;
        assert_eq!(log_10(x), 4);
    }

    #[test]
    fn test_split() {
        let x = 2024;
        assert_eq!(split_if_even(x), Some((20, 24)));

        let x = 99;
        assert_eq!(split_if_even(x), Some((9, 9)));

        let x = 2097446912;
        assert_eq!(split_if_even(x), Some((20974, 46912)));

        let x = 209744691;
        assert_eq!(split_if_even(x), None);

        let x = 9;
        assert_eq!(split_if_even(x), None);

        let x = 111;
        assert_eq!(split_if_even(x), None);
    }

    #[test]
    pub fn example_blink_1() {
        let curr = StoneLine {
            stones: vec![125, 17],
        };
        let expected = StoneLine {
            stones: vec![253000, 1, 7],
        };
        assert_eq!(curr.blink(), expected);
    }

    #[test]
    pub fn example_blink_2() {
        let curr = StoneLine {
            stones: vec![253000, 1, 7],
        };
        let expected = StoneLine {
            stones: vec![253, 0, 2024, 14168],
        };
        assert_eq!(curr.blink(), expected);
        assert_eq!(curr.blink(), expected);
    }

    #[test]
    pub fn example_blink_3() {
        let curr = StoneLine {
            stones: vec![253, 0, 2024, 14168],
        };
        let expected = StoneLine {
            stones: vec![512072, 1, 20, 24, 28676032],
        };
        assert_eq!(curr.blink(), expected);
    }

    #[test]
    pub fn example_blink_4() {
        let curr = StoneLine {
            stones: vec![512072, 1, 20, 24, 28676032],
        };
        let expected = StoneLine {
            stones: vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032],
        };
        assert_eq!(curr.blink(), expected);
    }

    #[test]
    pub fn example_blink_5() {
        let curr = StoneLine {
            stones: vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032],
        };
        let expected = StoneLine {
            stones: vec![1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32],
        };
        assert_eq!(curr.blink(), expected);
    }
    #[test]
    pub fn example_blink_6() {
        let curr = StoneLine {
            stones: vec![1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32],
        };
        let expected = StoneLine {
            stones: vec![
                2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6,
                0, 3, 2,
            ],
        };
        assert_eq!(curr.blink(), expected);
    }
}
