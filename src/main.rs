use colored::Colorize;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day: &String;
    match args.get(1) {
        Some(data) => day = data,
        None => {
            println!("Missing {} parameter!\nExiting...", "day".red(),);
            std::process::exit(1);
        }
    }
    let part: &String;
    match args.get(2) {
        Some(data) => part = data,
        None => {
            println!("Missing {} parameter!\nExiting...", "part".red(),);
            std::process::exit(1);
        }
    }
    let mut data_set = &String::from("example");
    match args.get(3) {
        Some(data) => data_set = data,
        None => println!(
            "Missing {} parameter. Using \"{}\"\n",
            "data_set".red(),
            "example".yellow(),
        ),
    }

    println!(
        "Doing Day {}, Part {} with {} data.\n",
        day.green(),
        part.green(),
        data_set.green(),
    );

    let data_content;
    match fs::read_to_string(format!("input/day_{day}/{data_set}.txt")) {
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

    println!("Data:\n\"{data_content}\"");

    // Do a thing
}
