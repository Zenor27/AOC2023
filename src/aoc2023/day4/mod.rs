use crate::aoc2023::day4::day4::{_solve1, _solve2};
use crate::aoc2023::result::AdventOfCodeResult;
use crate::aoc2023::tester::{test_part1, test_part2};

#[allow(clippy::module_inception)]
mod day4;

const DAY: u32 = 4;

pub fn solve1() -> AdventOfCodeResult {
    test_part1(DAY, _solve1, 21821)
}

pub fn solve2() -> AdventOfCodeResult {
    test_part2(DAY, _solve2, 5539496)
}
