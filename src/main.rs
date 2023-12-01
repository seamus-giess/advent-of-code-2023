use colored::Colorize;
use regex::Regex;
use std::collections::HashMap;
use std::time::Instant;

pub mod services;
use services::command_helpers;

fn main() {
    let (day, part, data_set) =
        command_helpers::get_arguments();

    println!(
        "Doing Day {}, Part {} with {} data.\n",
        day.green(),
        part.green(),
        data_set.green(),
    );

    let data =
        command_helpers::read_data_file(day, data_set);

    let exploded = data.split("\n");

    let start = Instant::now(); // benchmarking start
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
    let reg_ex: Regex = Regex::new(r"zero|one|two|three|four|five|six|seven|eight|nine|\d").unwrap();
    for line in exploded.into_iter() {
        let matches = reg_ex.captures(line).unwrap();
        let first = text_digits
            .get(matches.get(0).unwrap().as_str())
            .unwrap();
        let last = text_digits
            .get(
                matches
                    .get(matches.len() - 1)
                    .unwrap()
                    .as_str(),
            )
            .unwrap();

        let joined = (first * 10) + last;
        sum += joined;
    }
    let done = start.elapsed(); // benchmarking end

    println!(
        "{}",
        format!(" Solution: {} ", sum.to_string())
            .on_green()
    );
    println!(
        "Elapsed: {}",
        format!(" {:.2?} ", done).on_blue()
    );
}
