use std::fmt;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = File::open("inputs/day07.txt").unwrap();
    let reader = BufReader::new(file);

    let ans = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| Calibration::from(&line))
        .filter(|calibration| calibration.can_calibrate())
        .fold(0, |acc, calibration| acc + calibration.test_value);

    println!("Part 1 = {ans}");
}

fn part2() {
    let file = File::open("inputs/day07.txt").unwrap();
    let reader = BufReader::new(file);

    let ans = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| Calibration::from(&line))
        .filter(|calibration| calibration.can_calibrate_p2())
        .fold(0, |acc, calibration| acc + calibration.test_value);

    println!("Part 2 = {ans}");
}

#[derive(Debug)]
struct Calibration {
    test_value: u64,
    equation: Vec<u64>,
}

impl Calibration {
    pub fn from(line: &str) -> Self {
        let Some((v, list)) = line.split_once(':') else {
            panic!("unable to find test value in line")
        };
        let test_value = v.parse().expect("Unable to parse test value");

        let numbers = list.trim().split(' ');
        let mut equation = Vec::with_capacity(numbers.clone().count());
        for num in numbers {
            equation.push(num.parse().expect("Number in equation is not parseable"));
        }
        Self {
            test_value,
            equation,
        }
    }

    fn can_calibrate_recursive(&self, acc: u64, eq: &[u64]) -> bool {
        if acc > self.test_value {
            return false;
        }
        if eq.is_empty() {
            return self.test_value == acc;
        }
        if self.can_calibrate_recursive(acc * eq[0], &eq[1..]) {
            return true;
        }
        self.can_calibrate_recursive(acc + eq[0], &eq[1..])
    }

    pub fn can_calibrate(&self) -> bool {
        self.can_calibrate_recursive(self.equation[0], &self.equation[1..])
    }

    pub fn concat(x: u64, y: u64) -> u64 {
        let mut pow_10 = 1;
        while pow_10 <= y {
            pow_10 *= 10;
        }
        x * pow_10 + y
    }

    fn can_calibrate_recursive_p2(&self, acc: u64, eq: &[u64]) -> bool {
        if acc > self.test_value {
            // should not explore this execution path
            return false;
        }
        if eq.is_empty() {
            return self.test_value == acc;
        }
        if self.can_calibrate_recursive_p2(Self::concat(acc, eq[0]), &eq[1..]) {
            return true;
        }
        if self.can_calibrate_recursive_p2(acc * eq[0], &eq[1..]) {
            return true;
        }
        self.can_calibrate_recursive_p2(acc + eq[0], &eq[1..])
    }

    pub fn can_calibrate_p2(&self) -> bool {
        self.can_calibrate_recursive_p2(self.equation[0], &self.equation[1..])
    }
}

impl fmt::Display for Calibration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:", self.test_value)?;
        for x in &self.equation {
            write!(f, " {}", x)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn concat_tests() {
        assert_eq!(156, Calibration::concat(15, 6));
        assert_eq!(156, Calibration::concat(1, 56));
        assert_eq!(12345, Calibration::concat(12, 345));
        assert_eq!(
            12345,
            Calibration::concat(
                1,
                Calibration::concat(2, Calibration::concat(3, Calibration::concat(4, 5)))
            )
        );
        assert_eq!(11, Calibration::concat(1, 1));
        assert_eq!(110, Calibration::concat(1, 10));
        assert_eq!(1010, Calibration::concat(10, 10));
    }
}
