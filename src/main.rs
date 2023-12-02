use std::collections::HashMap;
use std::fs;

use crate::aoc2023::{day1, day2};

mod aoc2023;

type Solver = fn(String) -> u32;
type Solvers = [Solver; 2];

fn main() {
    let day_to_solvers = HashMap::from([
        ("day1", [day1::solve1, day1::solve2] as Solvers),
        ("day2", [day2::solve1, day2::solve2] as Solvers),
    ]);

    let args: Vec<String> = std::env::args().collect();
    let day = args.get(1).expect("Please provide a day to solve");

    let solvers = day_to_solvers
        .get(day.as_str())
        .expect(&format!("{day} not found"));
    solvers.iter().enumerate().for_each(|(index, solver)| {
        let input_index = index + 1;
        let path = format!("src/aoc2023/{day}/input{input_index}.txt");
        let input = fs::read_to_string(&path)
            .expect(&format!("Something went wrong reading the file {path}"));
        let solution = solver(input);
        println!("Solution {} for day {}: {}", index, day, solution);
    });
}
