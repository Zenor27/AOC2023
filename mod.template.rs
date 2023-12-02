use crate::aoc2023::day{{day}}::day{{day}}::{_solve1, _solve2};
use crate::aoc2023::result::AdventOfCodeResult;
use crate::aoc2023::tester::{test_part1, test_part2};

mod day{{day}};

const DAY: u32 = {{day}};

pub fn solve1() -> AdventOfCodeResult {
    test_part1(DAY, _solve1, 1)
}

pub fn solve2() -> AdventOfCodeResult {
    test_part2(DAY, _solve2, 1)
}
