use colored::Colorize;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::time::Instant;

fn main() {
    // benchmarking
    let now = Instant::now();

    let args: Vec<String> = env::args().collect();

    let day: &String;
    match args.get(1) {
        Some(data) => day = data,
        None => {
            println!(
                "Missing {} parameter!\nExiting...",
                "day".red(),
            );
            std::process::exit(1);
        }
    }
    let part: &String;
    match args.get(2) {
        Some(data) => part = data,
        None => {
            println!(
                "Missing {} parameter!\nExiting...",
                "part".red(),
            );
            std::process::exit(1);
        }
    }
    let mut data_set = &String::from("example");
    match args.get(3) {
        Some(data) => data_set = data,
        None => {
            println!(
                "Missing {} parameter. Using \"{}\"\n",
                "data_set".red(),
                "example".yellow(),
            )
        }
    }

    println!(
        "Doing Day {}, Part {} with {} data.\n",
        day.green(),
        part.green(),
        data_set.green(),
    );

    let data_content;
    match fs::read_to_string(format!(
        "input/day_{day}/{data_set}.txt"
    )) {
        Ok(content) => data_content = content,
        Err(_) => {
            println!(
                "Error reading file: \"input/{}/{}\"\nDoes it exist?",
                format!("day_{day}").red(),
                format!("{data_set}.txt").red()
            );
            std::process::exit(1);
        }
    }

    println!("Data:\n\"{data_content}\"\n");

    let exploded = data_content.split("\n");

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
    println!("{}", sum);

    // let mut sum: u32 = 0;
    // exploded.for_each(|x| {
    //     let first = x.chars().next().unwrap().to_digit(10).unwrap();
    //     let last = x.chars().last().unwrap().to_digit(10).unwrap();
    //     sum += (first * 10) + last;
    //     println!("{}, {}, {}", sum, (first * 10), last)
    // });
    // println!("{}", sum);
    // Do a thing

    println!("Elapsed: {:.2?}", now.elapsed());
}
