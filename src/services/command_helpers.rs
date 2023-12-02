use colored::Colorize;
use std::env;
use std::fs;
use std::process;

/**
 * Get the arguments from the command line.
 *
 * @return (day, part, data_set)
 */
pub fn get_arguments() -> (String, String, String) {
    let args: Vec<String> = env::args().collect();

    let day: &String;
    match args.get(1) {
        Some(data) => day = data,
        None => {
            println!("Missing {} parameter!\nExiting...", "day".red(),);
            process::exit(1);
        }
    }

    let part: &String;
    match args.get(2) {
        Some(data) => part = data,
        None => {
            println!("Missing {} parameter!\nExiting...", "part".red(),);
            process::exit(1);
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

    return (day.to_string(), part.to_string(), data_set.to_string());
}

pub fn read_data_file(day: &String, data_set: &String) -> String {
    match fs::read_to_string(format!("input/day_{day}/{data_set}.txt")) {
        Ok(content) => return content,
        Err(_) => {
            println!(
                "Error reading file: \"input/{}/{}\"\nDoes it exist?",
                format!("day_{day}").red(),
                format!("{data_set}.txt").red()
            );
            std::process::exit(1);
        }
    }
}
