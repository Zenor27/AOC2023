use std::collections::HashMap;

pub fn solve1(input: String) -> u32 {
    let input: Vec<u32> = input.lines().map(|line| {
        let chars = line.chars().filter(|c| c.is_digit(10)).map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>();
        let mut chars = chars.iter();
        let first = chars.next();
        let last = chars.last();
        return first.or(last).unwrap() * 10 + last.or(first).unwrap();
    }).collect();
    return input.iter().sum();
}

pub fn solve2(input: String) -> u32 {
    let digit_map = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("zero", 0),
    ]);

    let input = input.lines().map(|line| {
        // Double each digit word
        let line = digit_map.keys().fold(line.to_string(), |acc, word| {
            acc.replace(word, &format!("{word}{word}"))
        });
        // Replace each digit word with the digit
        return digit_map.keys().fold(line, |acc, word| {
            acc.replace(word, &format!("{}", digit_map.get(word).unwrap()))
        });
    }).collect::<Vec<String>>().join("\n");

    return solve1(input);
}