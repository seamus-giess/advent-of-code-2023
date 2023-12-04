use std::collections::{HashMap, HashSet};

pub fn solve(data: &String) -> String {
    let card_strings = data.split('\n');
    let mut cards = Vec::new();
    card_strings.for_each(|card| {
        let (_card_id, card_numbers_string) = match card.split_once(": ") {
            Some((card_label, card_numbers_string)) => (
                card_label
                    .split_whitespace()
                    .last()
                    .unwrap()
                    .parse::<usize>()
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
        cards.push((winning_numbers, your_numbers));
    });
    let mut card_queue_template: HashMap<usize, i32> = HashMap::new();
    (1..cards.len() + 1).for_each(|key| {
        card_queue_template.insert(key, 1);
    });
    let mut index_tracker = 1;
    let sum = cards
        .into_iter()
        .fold(
            card_queue_template,
            |card_queue, (winning_numbers, your_numbers)| {
                let index = index_tracker.clone();
                index_tracker += 1;

                let mut new_cards = card_queue.clone();

                let current_cards_count: i32 = match new_cards.get(&index) {
                    Some(count) => count.clone(),
                    None => 1,
                };

                let wins: i32 = winning_numbers.intersection(&your_numbers).count() as i32;
                ((index + 1)..(index + 1 + wins as usize))
                    .collect::<Vec<usize>>()
                    .into_iter()
                    .for_each(|index| {
                        new_cards.insert(
                            index,
                            match new_cards.get(&index) {
                                Some(count) => count + current_cards_count,
                                None => 0 + current_cards_count,
                            },
                        );
                    });

                return new_cards;
            },
        )
        .into_iter()
        .fold(0, |sum, (_index, count)| {
            return sum + count;
        });
    assert_eq!(sum, 9881048);
    return sum.to_string();
}
