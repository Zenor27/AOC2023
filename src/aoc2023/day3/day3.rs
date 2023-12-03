use std::collections::HashMap;

use glam::IVec2;

fn get_number_to_positions(input: &str) -> HashMap<u32, Vec<IVec2>> {
    let mut number_to_positions: HashMap<u32, Vec<IVec2>> = HashMap::new();
    let mut add_number_positions = |number: u32, x: usize, y: usize| {
        if number == 0 {
            return;
        }
        let indexes = number_to_positions.entry(number).or_default();
        let number_len = number.to_string().len();
        for i in 0..number_len {
            indexes.push(IVec2::new((x - number_len + i) as i32, y as i32));
        }
    };
    input.lines().enumerate().for_each(|(y, line)| {
        let mut number = 0;
        let mut x = 0;
        for c in line.chars() {
            if c.is_ascii_digit() {
                number = number * 10 + c.to_digit(10).unwrap();
            } else {
                add_number_positions(number, x, y);
                number = 0;
            }
            x += 1;
        }
        add_number_positions(number, x, y);
    });

    number_to_positions
}

fn is_adjacent(positions: &[IVec2], position: &IVec2) -> bool {
    positions
        .iter()
        .any(|p| (p.x - position.x).abs() == 1 && (p.y - position.y).abs() == 1)
        || positions
            .iter()
            .any(|p| (p.x - position.x).abs() + (p.y - position.y).abs() == 1)
}

fn get_adjacent_numbers(
    number_to_positions: &HashMap<u32, Vec<IVec2>>,
    symbols_positions: &[IVec2],
) -> Vec<Vec<u32>> {
    symbols_positions
        .iter()
        .map(|position| {
            number_to_positions
                .iter()
                .filter_map(move |(number, positions)| {
                    if is_adjacent(positions, position) {
                        Some(*number)
                    } else {
                        None
                    }
                })
                .collect()
        })
        .collect()
}

pub(super) fn _solve1(input: String) -> u32 {
    let symbols_positions = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c != '.' && !c.is_ascii_digit() {
                    Some(IVec2::new(x as i32, y as i32))
                } else {
                    None
                }
            })
        })
        .collect::<Vec<IVec2>>();

    let number_to_positions = get_number_to_positions(&input);
    return get_adjacent_numbers(&number_to_positions, &symbols_positions)
        .iter()
        .flatten()
        .sum();
}

pub(super) fn _solve2(input: String) -> u32 {
    let number_to_positions = get_number_to_positions(&input);
    let gear_positions = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c == '*' {
                    Some(IVec2::new(x as i32, y as i32))
                } else {
                    None
                }
            })
        })
        .filter(|position| {
            number_to_positions
                .values()
                .filter(|positions| is_adjacent(positions, position))
                .count()
                >= 2
        })
        .collect::<Vec<IVec2>>();

    get_adjacent_numbers(&number_to_positions, &gear_positions)
        .into_iter()
        .map(|numbers| numbers.into_iter().reduce(|a, b| a * b).unwrap())
        .sum()
}
