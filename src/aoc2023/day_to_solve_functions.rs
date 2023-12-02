use phf::{phf_map, Map};

use crate::aoc2023;

pub type DayToSolveFunctions = Map<&'static str, [fn() -> aoc2023::result::AdventOfCodeResult; 2]>;

pub static DAY_TO_SOLVE_FUNCTIONS: DayToSolveFunctions = phf_map! {
    "day1" => [aoc2023::day1::solve1, aoc2023::day1::solve2],
    "day2" => [aoc2023::day2::solve1, aoc2023::day2::solve2],
};
