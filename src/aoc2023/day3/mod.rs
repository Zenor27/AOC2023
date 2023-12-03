use crate::aoc2023::day3::day3::{_solve1, _solve2};
use crate::aoc2023::result::AdventOfCodeResult;
use crate::aoc2023::tester::{test_part1, test_part2};

mod day3;

const DAY: u32 = 3;

pub fn solve1() -> AdventOfCodeResult {
    test_part1(DAY, _solve1, 532331)
}

pub fn solve2() -> AdventOfCodeResult {
    test_part2(DAY, _solve2, 82301120)
}
