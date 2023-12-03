use std::collections::HashMap;

type Position = (usize, usize);

fn get_number_to_positions(input: String) -> HashMap<u32, Vec<Position>> {
    let mut number_to_positions: HashMap<u32, Vec<Position>> = HashMap::new();
    let mut add_number_positions = |number: u32, x: usize, y: usize| {
        if number == 0 {
            return;
        }
        let indexes = number_to_positions.entry(number).or_default();
        let number_len = number.to_string().len();
        for i in 0..number_len {
            indexes.push((x - number_len + i, y));
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

fn is_adjacent(positions: &[Position], x: usize, y: usize) -> bool {
    [
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ]
    .iter()
    .any(|position| positions.contains(position))
}

fn get_adjacent_numbers(
    number_to_positions: &HashMap<u32, Vec<Position>>,
    symbols_positions: &[Position],
) -> Vec<Vec<u32>> {
    symbols_positions
        .iter()
        .map(|(x, y)| {
            number_to_positions
                .iter()
                .filter_map(move |(number, positions)| {
                    if is_adjacent(positions, *x, *y) {
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
                    Some((x, y))
                } else {
                    None
                }
            })
        })
        .collect::<Vec<Position>>();

    let number_to_positions = get_number_to_positions(input);
    return get_adjacent_numbers(&number_to_positions, &symbols_positions)
        .iter()
        .flatten()
        .sum();
}

pub(super) fn _solve2(input: String) -> u32 {
    let gear_positions = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(
                move |(x, c)| {
                    if c == '*' {
                        Some((x, y))
                    } else {
                        None
                    }
                },
            )
        })
        .collect::<Vec<Position>>();

    let number_to_positions = get_number_to_positions(input);

    let gear_positions: Vec<_> = gear_positions
        .into_iter()
        .filter(|(x, y)| {
            number_to_positions
                .values()
                .filter(|positions| is_adjacent(positions, *x, *y))
                .count()
                >= 2
        })
        .collect();

    get_adjacent_numbers(&number_to_positions, &gear_positions)
        .into_iter()
        .map(|numbers| numbers.into_iter().reduce(|a, b| a * b).unwrap())
        .sum()
}
