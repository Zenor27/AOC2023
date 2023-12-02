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

fn parse_games(input: String) -> Vec<Game> {
    return input.lines().map(|line| {
        let game_id = line.split(":").next().unwrap().split(" ").last().unwrap().parse::<u32>().unwrap();
        let sets = line.split(": ").last().unwrap().split("; ").map(|set| {
            let blue_count = set.split(" blue").next().unwrap().split(" ").last().unwrap().parse::<u32>().unwrap_or(0);
            let red_count = set.split(" red").next().unwrap().split(" ").last().unwrap().parse::<u32>().unwrap_or(0);
            let green_count = set.split(" green").next().unwrap().split(" ").last().unwrap().parse::<u32>().unwrap_or(0);
            return Set {
                blue: blue_count,
                red: red_count,
                green: green_count,
            };
        }).collect::<Vec<Set>>();
        let game = Game {
            id: game_id,
            sets,
        };

        return game;
    }).collect::<Vec<Game>>();
}

pub fn solve1(input: String) -> u32 {
    return parse_games(input).iter().filter(|game| {
        game.max_blue_count() <= 14 && game.max_green_count() <= 13 && game.max_red_count() <= 12
    }).map(|game| game.id).sum::<u32>();
}

pub fn solve2(input: String) -> u32 {
    return parse_games(input).iter().map(|game| game.power()).sum::<u32>();
}