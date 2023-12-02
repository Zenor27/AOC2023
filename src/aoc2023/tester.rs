use std::fs;

use crate::aoc2023::result::{AdventOfCodeMismatch, AdventOfCodeOk, AdventOfCodeResult};

fn tester(
    day: u32,
    part: u32,
    func: fn(input: String) -> u32,
    expected: u32,
) -> AdventOfCodeResult {
    let path = format!("src/aoc2023/day{day}/input{part}.txt");
    let input =
        fs::read_to_string(&path).expect(&format!("Something went wrong reading the file {path}"));
    let value = func(input);
    return if value == expected {
        Ok(AdventOfCodeOk { result: value })
    } else {
        Err(AdventOfCodeMismatch {
            expected,
            actual: value,
        })
    };
}

pub fn test_part1(day: u32, func: fn(input: String) -> u32, expected: u32) -> AdventOfCodeResult {
    tester(day, 1, func, expected)
}

pub fn test_part2(day: u32, func: fn(input: String) -> u32, expected: u32) -> AdventOfCodeResult {
    tester(day, 2, func, expected)
}
