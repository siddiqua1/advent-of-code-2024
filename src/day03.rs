use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = File::open("inputs/day03-p1.txt").unwrap();
    let reader = BufReader::new(file);

    let ans = reader
        .lines()
        .map(|line| line.unwrap())
        .fold(0, |acc, line| acc + eval_line(&line));

    println!("Part 1 = {ans}");
}

fn part2() {
    let file = File::open("inputs/day03-p1.txt").unwrap();
    let reader = BufReader::new(file);

    let (ans, _) = reader.lines().map(|line| line.unwrap()).fold(
        (0, Enabler::default()),
        |(acc, fsm), line| {
            let (x, fsm) = eval_line_with_enable(&line, fsm);
            (acc + x, fsm)
        },
    );

    println!("Part 2 = {ans}");
}

fn eval_line(line: &str) -> i64 {
    let mut acc = 0;
    let mut state = MulStateMachine::None;
    for c in line.chars() {
        state = state.step(c);
        acc += state.value();
    }
    acc
}

fn eval_line_with_enable(line: &str, state: Enabler) -> (i64, Enabler) {
    let mut acc = 0;
    let mut state = state;
    for c in line.chars() {
        state = state.step(c);
        acc += state.value();
        // println!("{acc}\t{:?}", state);
    }
    (acc, state)
}

#[derive(Debug)]
enum MulStateMachine {
    None,
    M,
    U,
    L,
    Open,
    FirstNum(i64),
    SecondNum(i64, i64),
    Finished(i64, i64),
}

impl MulStateMachine {
    pub fn value(&self) -> i64 {
        if let MulStateMachine::Finished(a, b) = self {
            return a * b;
        }
        0
    }
    pub fn step(self, c: char) -> Self {
        match self {
            MulStateMachine::None => {
                if c == 'm' {
                    return MulStateMachine::M;
                }
            }
            MulStateMachine::M => {
                if c == 'u' {
                    return MulStateMachine::U;
                }
            }
            MulStateMachine::U => {
                if c == 'l' {
                    return MulStateMachine::L;
                }
            }
            MulStateMachine::L => {
                if c == '(' {
                    return MulStateMachine::Open;
                }
            }
            MulStateMachine::Open => {
                if let Some(d) = c.to_digit(10) {
                    return MulStateMachine::FirstNum(d as i64);
                }
            }
            MulStateMachine::FirstNum(x) => {
                if x < 1000 {
                    if let Some(d) = c.to_digit(10) {
                        return MulStateMachine::FirstNum(10 * x + (d as i64));
                    }
                    if c == ',' {
                        return MulStateMachine::SecondNum(x, 0);
                    }
                }
            }
            MulStateMachine::SecondNum(x, y) => {
                if y < 1000 {
                    if let Some(d) = c.to_digit(10) {
                        return MulStateMachine::SecondNum(x, 10 * y + (d as i64));
                    }
                    if c == ')' {
                        return MulStateMachine::Finished(x, y);
                    }
                }
            }
            MulStateMachine::Finished(_, _) => {}
        } // no valid state transition, fall back to starting
        if c == 'm' {
            return MulStateMachine::M;
        }
        MulStateMachine::None
    }
}

#[derive(Debug)]
enum EnablerFSM {
    None,
    D,
    O,
    DoOpen,
    DoClose,
    N,
    Apostrophe,
    T,
    DontOpen,
    DontClose,
}

#[derive(Debug)]
enum Enabler {
    Do(EnablerFSM, MulStateMachine),
    Dont(EnablerFSM),
}

impl EnablerFSM {
    pub fn state(&self) -> Option<bool> {
        match self {
            EnablerFSM::DoClose => Some(true),
            EnablerFSM::DontClose => Some(false),
            _ => None,
        }
    }
    pub fn step(self, c: char) -> Self {
        match self {
            EnablerFSM::None => {}
            EnablerFSM::D => {
                if c == 'o' {
                    return EnablerFSM::O;
                }
            }
            EnablerFSM::O => {
                if c == 'n' {
                    return EnablerFSM::N;
                }
                if c == '(' {
                    return EnablerFSM::DoOpen;
                }
            }
            EnablerFSM::DoOpen => {
                if c == ')' {
                    return EnablerFSM::DoClose;
                }
            }
            EnablerFSM::N => {
                if c == '\'' {
                    return EnablerFSM::Apostrophe;
                }
            }
            EnablerFSM::Apostrophe => {
                if c == 't' {
                    return EnablerFSM::T;
                }
            }
            EnablerFSM::T => {
                if c == '(' {
                    return EnablerFSM::DontOpen;
                }
            }
            EnablerFSM::DontOpen => {
                if c == ')' {
                    return EnablerFSM::DontClose;
                }
            }
            EnablerFSM::DoClose | EnablerFSM::DontClose => {}
        }
        if c == 'd' {
            return EnablerFSM::D;
        }
        EnablerFSM::None
    }
}

impl Enabler {
    pub fn default() -> Self {
        Enabler::Do(EnablerFSM::None, MulStateMachine::None)
    }
    pub fn value(&self) -> i64 {
        match self {
            Enabler::Do(_, mul) => mul.value(),
            Enabler::Dont(_) => 0,
        }
    }

    pub fn step(self, c: char) -> Self {
        match self {
            Enabler::Do(fsm, mul) => {
                let fsm = fsm.step(c);
                if let Some(false) = fsm.state() {
                    return Enabler::Dont(fsm);
                }
                Enabler::Do(fsm, mul.step(c))
            }
            Enabler::Dont(fsm) => {
                let fsm = fsm.step(c);
                if let Some(true) = fsm.state() {
                    let mul = MulStateMachine::None;
                    return Enabler::Do(fsm, mul.step(c));
                }
                Enabler::Dont(fsm)
            }
        }
    }
}
