use std::collections::HashMap;

pub fn solve(data: &String) -> String {
    let exploded = data.split("\n");

    let text_digits = HashMap::from([
        ("zero", 0),
        ("0", 0),
        ("one", 1),
        ("1", 1),
        ("two", 2),
        ("2", 2),
        ("three", 3),
        ("3", 3),
        ("four", 4),
        ("4", 4),
        ("five", 5),
        ("5", 5),
        ("six", 6),
        ("6", 6),
        ("seven", 7),
        ("7", 7),
        ("eight", 8),
        ("8", 8),
        ("nine", 9),
        ("9", 9),
    ]);

    let mut sum = 0;
    for line in exploded.into_iter() {
        let mut found_digits = HashMap::new();
        for digit in text_digits.clone().into_iter() {
            if let Some(first_index) = line.find(digit.0) {
                found_digits.insert(first_index, digit.1);
            };
            if let Some(last_index) = line.rfind(digit.0) {
                found_digits.insert(last_index, digit.1);
            };
        }
        let lowest = found_digits
            .get(found_digits.keys().min().unwrap())
            .unwrap();
        let highest = found_digits
            .get(found_digits.keys().max().unwrap())
            .unwrap();

        let joined = (lowest * 10) + highest;
        sum += joined;
    }

    return sum.to_string();
}
