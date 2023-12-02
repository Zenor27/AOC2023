struct Set {
    blue: u32,
    red: u32,
    green: u32,
}

struct Game {
    id: u32,
    sets: Vec<Set>,
}

impl Game {
    fn max_blue_count(&self) -> u32 {
        self.sets.iter().map(|set| set.blue).max().unwrap_or(0)
    }

    fn max_red_count(&self) -> u32 {
        self.sets.iter().map(|set| set.red).max().unwrap_or(0)
    }

    fn max_green_count(&self) -> u32 {
        self.sets.iter().map(|set| set.green).max().unwrap_or(0)
    }

    fn power(&self) -> u32 {
        self.max_green_count() * self.max_red_count() * self.max_blue_count()
    }
}

fn get_color_count(set: &str, color: &str) -> u32 {
    set.split(&format!(" {}", color))
        .next()
        .unwrap()
        .split(" ")
        .last()
        .unwrap()
        .parse::<u32>()
        .unwrap_or(0)
}

fn get_game_id(line: &str) -> u32 {
    line.split(":")
        .next()
        .unwrap()
        .split(" ")
        .last()
        .unwrap()
        .parse::<u32>()
        .unwrap()
}

fn get_sets(line: &str) -> Vec<Set> {
    line.split(": ")
        .last()
        .unwrap()
        .split("; ")
        .map(|set| {
            let blue_count = get_color_count(set, "blue");
            let red_count = get_color_count(set, "red");
            let green_count = get_color_count(set, "green");
            return Set {
                blue: blue_count,
                red: red_count,
                green: green_count,
            };
        })
        .collect::<Vec<Set>>()
}

fn parse_games(input: String) -> Vec<Game> {
    return input
        .lines()
        .map(|line| {
            let id = get_game_id(line);
            let sets = get_sets(line);
            let game = Game { id, sets };

            return game;
        })
        .collect::<Vec<Game>>();
}

pub(super) fn _solve1(input: String) -> u32 {
    return parse_games(input)
        .iter()
        .filter(|game| {
            game.max_blue_count() <= 14
                && game.max_green_count() <= 13
                && game.max_red_count() <= 12
        })
        .map(|game| game.id)
        .sum::<u32>();
}

pub(super) fn _solve2(input: String) -> u32 {
    return parse_games(input)
        .iter()
        .map(|game| game.power())
        .sum::<u32>();
}
