use phf::{phf_map, Map};

use crate::aoc2023::{day1, day2, result};

pub type DayToSolveFunctions = Map<&'static str, [fn() -> result::AdventOfCodeResult; 2]>;

pub static DAY_TO_SOLVE_FUNCTIONS: DayToSolveFunctions = phf_map! {
    "day1" => [day1::solve1, day1::solve2],
    "day2" => [day2::solve1, day2::solve2],
};
