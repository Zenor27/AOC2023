use crate::aoc2023::day2::day2::{_solve1, _solve2};
use crate::aoc2023::result::AdventOfCodeResult;
use crate::aoc2023::tester::{test_part1, test_part2};

#[allow(clippy::module_inception)]
mod day2;

const DAY: u32 = 2;

pub fn solve1() -> AdventOfCodeResult {
    test_part1(DAY, _solve1, 2283)
}

pub fn solve2() -> AdventOfCodeResult {
    test_part2(DAY, _solve2, 78669)
}
