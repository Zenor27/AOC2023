use phf::{phf_map, Map};

pub(super) fn _solve1(input: String) -> u32 {
    return input
        .lines()
        .map(|line| {
            let chars = line.chars().filter_map(|c| c.to_digit(10));
            let first = chars.clone().next();
            let last = chars.clone().last();
            return first.or(last).unwrap() * 10 + last.or(first).unwrap();
        })
        .collect::<Vec<u32>>()
        .iter()
        .sum();
}

static DIGIT_MAP: Map<&str, i32> = phf_map! {
    "one" => 1,
    "two" => 2,
    "three" => 3,
    "four" => 4,
    "five" => 5,
    "six" => 6,
    "seven" => 7,
    "eight" => 8,
    "nine" => 9,
    "zero" => 0,
};

pub(super) fn _solve2(input: String) -> u32 {
    let input = input
        .lines()
        .map(|line| {
            // Double each digit word
            let line = DIGIT_MAP.keys().fold(line.to_string(), |acc, word| {
                acc.replace(word, &format!("{word}{word}"))
            });
            // Replace each digit word with the digit
            return DIGIT_MAP
                .entries()
                .fold(line, |acc, (digit_as_word, digit)| {
                    acc.replace(digit_as_word, digit.to_string().as_str())
                });
        })
        .collect::<Vec<String>>()
        .join("\n");

    return _solve1(input);
}
