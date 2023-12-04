use std::collections::HashSet;

pub fn solve(data: &String) -> String {
    let mut sum = 0;
    let cards = data.split('\n');
    cards.for_each(|card| {
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
        let wins: u32 = winning_numbers.intersection(&your_numbers).count() as u32;
        println!(
            "game: {}, wins: {}, intersects: {:?}, score {}",
            card_id,
            wins,
            winning_numbers.intersection(&your_numbers),
            1 * (2 as u32).pow(wins - 1)
        );
        if wins >= 1 {
            sum += 1 * (2 as u32).pow(wins - 1);
        }
    });
    return sum.to_string();
}
