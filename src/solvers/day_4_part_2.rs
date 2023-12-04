use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

pub fn solve(data: &String) -> String {
    let mut sum = 0;
    let card_strings = data.split('\n');
    let mut won_cards: VecDeque<i32> = VecDeque::new();
    let mut cards = HashMap::new();
    card_strings.for_each(|card| {
        let (card_id, card_numbers_string) = match card.split_once(": ") {
            Some((card_label, card_numbers_string)) => (
                card_label
                    .split_whitespace()
                    .last()
                    .unwrap()
                    .parse::<i32>()
                    .unwrap(),
                card_numbers_string,
            ),
            None => panic!("Can't split card and numbers!"),
        };
        let (winning_numbers, your_numbers) = match card_numbers_string.split_once(" | ") {
            Some((wining_numbers_string, your_numbers_string)) => (
                HashSet::<&str>::from_iter(wining_numbers_string.split_whitespace().into_iter()),
                HashSet::<&str>::from_iter(your_numbers_string.split_whitespace().into_iter()),
            ),
            _ => panic!("Can't split card and numbers!"),
        };
        cards.insert(card_id, (winning_numbers, your_numbers));
    });
    let mut won_cards_count: HashMap<i32, i32> = HashMap::new();
    let mut won_cards: VecDeque<i32> = VecDeque::new();
    cards.keys().for_each(|key| {
        won_cards.push_back(key.clone());
    });
    while won_cards.len() > 0 {
        let next_card = won_cards.pop_front().unwrap();
        won_cards_count.insert(
            next_card,
            match won_cards_count.get(&next_card) {
                Some(count) => count + 1,
                None => 1,
            },
        );
        let (winning_numbers, your_numbers) = cards.get(&next_card).unwrap();
        let wins = winning_numbers.intersection(your_numbers);
        let wins_count: i32 = wins.count() as i32;
        let mut counter = 0;
        while counter < wins_count {
            counter += 1;
            won_cards.push_front(next_card + counter);
        }
    }
    won_cards_count.into_iter().for_each(|(_key, cards_count)| {
        sum += cards_count;
    });
    return sum.to_string();
}
