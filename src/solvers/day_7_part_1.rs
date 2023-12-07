use std::{collections::HashMap, str::FromStr};

use itertools::Itertools;

struct Object {
    hands: Vec<(String, i32)>,
}
impl Object {}

struct StringParseError;
impl FromStr for Object {
    type Err = StringParseError;
    fn from_str(data: &str) -> Result<Self, Self::Err> {
        let hands_and_bids = data.split('\n');

        let mut hands = Vec::new();
        hands_and_bids.for_each(|string| {
            let (hand, bid) = string.split_once(' ').unwrap();
            hands.push((hand.to_string(), bid.parse().unwrap()));
        });

        return Ok(Object { hands: hands });
    }
}

pub fn solve(data: &String) -> String {
    let game = match data.parse::<Object>() {
        Ok(object) => object,
        Err(_) => panic!("Could not parse object"),
    };

    let rankings = Vec::from([
        "high_card",
        "one_pair",
        "two_pair",
        "three_of_a_kind",
        "full_house",
        "four_of_a_kind",
        "five_of_a_kind",
    ]);

    let mut winners: Vec<(String, String, i32)> = Vec::new();
    game.hands.into_iter().for_each(|(hand, bid)| {
        let hand_string = hand.clone();
        winners.push((determine_hand_set(hand_string), hand, bid));
    });

    let mut sorted_winners: Vec<(String, String, i32)> = Vec::new();
    rankings.into_iter().for_each(|ranking| {
        let ranking_winners: Vec<(String, String, i32)> = winners
            .clone()
            .into_iter()
            .filter(|(rank, _hand, _)| rank.as_str() == ranking)
            .collect();

        ranking_winners
            .into_iter()
            .sorted_by(|(_, a, _), (_, b, _)| {
                let mut a_chars = a.chars();
                let mut b_chars = b.chars();
                loop {
                    let a_char = get_label_val(a_chars.next().unwrap());
                    let b_char = get_label_val(b_chars.next().unwrap());
                    if a_char == b_char {
                        continue;
                    }
                    return a_char.cmp(&b_char);
                }
            })
            .for_each(|(rank, hand, bid)| {
                sorted_winners.push((rank, hand, bid));
            });
    });

    let mut running_rank = 0;
    let sum = sorted_winners
        .into_iter()
        .fold(0, |sum, (rank, hand, bid)| {
            running_rank += 1;
            println!(
                "Hand: {:?} with bid {:?} won {:?} as {:?}",
                hand,
                bid,
                bid * running_rank,
                rank,
            );
            return sum + (bid * running_rank);
        });

    return sum.to_string();
}

fn determine_hand_set(hand: String) -> String {
    let labels = hand.chars();
    let mut label_counts: HashMap<char, i32> = HashMap::new();
    labels.for_each(|char| {
        label_counts.insert(
            char,
            match label_counts.get(&char) {
                Some(val) => val,
                None => &0,
            } + 1,
        );
    });

    match label_counts
        .clone()
        .into_iter()
        .filter(|(char, _count)| char == &'J')
        .next()
    {
        Some((char, j_count)) => {
            let mut counts_iter =
                label_counts
                    .into_iter()
                    .sorted_by(|(char_a, val_a), (char_b, val_b)| {
                        if char_a == &'J' {
                            return 2.cmp(&1);
                        }
                        if char_b == &'J' {
                            return 1.cmp(&2);
                        }

                        return val_b.cmp(val_a);
                    });
            let (add_to, count) = counts_iter.next().unwrap();
            label_counts = counts_iter.collect();
            println!("{:?} {:?}", add_to, count + j_count);
            if add_to != char {
                label_counts.remove(&'J');
                // TODO handle if J
                label_counts.insert(add_to, count + j_count);
            } else if add_to == char {
                println!("{:?} {:?} {:?}", add_to, char, label_counts);
                label_counts.insert(char, j_count);
            }
        }
        None => (),
    };

    return match label_counts.len() {
        1 => "five_of_a_kind".to_string(), // Five of a kind
        2 => {
            let (_, highest_count) = label_counts
                .clone()
                .into_iter()
                .max_by(|(_, a), (_, b)| {
                    return a.cmp(b);
                })
                .unwrap();

            if highest_count == 4 {
                // Four of a kind
                return "four_of_a_kind".to_string();
            } else {
                // Full house
                return "full_house".to_string();
            }
        }
        3 => {
            let iter = label_counts.clone().into_iter();
            let (index, highest_count) = iter
                .max_by(|(_, a), (_, b)| {
                    return a.cmp(b);
                })
                .unwrap();
            label_counts.remove(&index);

            let (_, _second_count) = label_counts
                .into_iter()
                .max_by(|(_, a), (_, b)| {
                    return a.cmp(b);
                })
                .unwrap();

            if highest_count == 3 {
                // Three of a kind
                return "three_of_a_kind".to_string();
            } else {
                // Two pair
                return "two_pair".to_string();
            }
        }
        4 => "one_pair".to_string(),
        _ => "high_card".to_string(), // High card
    };
}

fn get_label_val(char: char) -> i32 {
    match char {
        'J' => -1,
        '2' => 0,
        '3' => 1,
        '4' => 2,
        '5' => 3,
        '6' => 4,
        '7' => 5,
        '8' => 6,
        '9' => 7,
        'T' => 8,
        'Q' => 10,
        'K' => 11,
        'A' => 12,
        _ => panic!("{}", format!("Couldn't find char: {}", char)),
    }
}
