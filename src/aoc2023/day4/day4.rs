use std::collections::HashSet;

use itertools::Itertools;

struct Card {
    winning_numbers: HashSet<u32>,
    picked_numbers: HashSet<u32>,
}

impl Card {
    pub fn get_intersections(&self) -> usize {
        self.winning_numbers
            .intersection(&self.picked_numbers)
            .count()
    }

    pub fn points(&self) -> u32 {
        let intersection_count = self.get_intersections();
        if intersection_count == 0 {
            return 0;
        }
        (1..intersection_count).fold(1, |acc, _| acc * 2)
    }
}

fn get_cards(input: &str) -> Vec<Card> {
    input
        .lines()
        .map(|line| {
            let (winning_numbers, picked_numbers) = line
                .split(": ")
                .last()
                .unwrap()
                .split(" | ")
                .collect_tuple()
                .unwrap();
            let winning_numbers = winning_numbers
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|s| s.trim().parse().unwrap())
                .collect::<HashSet<u32>>();
            let picked_numbers = picked_numbers
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|s| s.trim().parse().unwrap())
                .collect::<HashSet<u32>>();
            Card {
                winning_numbers,
                picked_numbers,
            }
        })
        .collect()
}

pub(super) fn _solve1(input: String) -> u32 {
    return get_cards(&input).iter().map(|card| card.points()).sum();
}

pub(super) fn _solve2(input: String) -> u32 {
    let cards = get_cards(&input);
    let total_card = cards.len();
    let mut instances = vec![1; total_card];

    cards.iter().enumerate().for_each(|(card_idx, card)| {
        let intersections = card.get_intersections();
        let current_card_instances = instances[card_idx];
        for intersection in 0..intersections {
            let next_card_idx = (card_idx + intersection + 1) % total_card;
            let next_card_instances = instances[next_card_idx];
            instances[next_card_idx] = next_card_instances + current_card_instances;
        }
    });

    return instances.iter().sum();
}
