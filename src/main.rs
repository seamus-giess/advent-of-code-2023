use colored::Colorize;
use std::time::Instant;

pub mod services;
use services::command_helpers;

pub mod solvers;

fn main() {
    let (day, part, data_set) = command_helpers::get_arguments();

    println!(
        "Doing Day {}, Part {} with {} data.\n",
        day.green(),
        part.green(),
        data_set.green(),
    );

    let data = command_helpers::read_data_file(&day, &data_set);

    let start = Instant::now(); // benchmarking start
    let resolution = solvers::solve(&day, &part, &data);
    let done = start.elapsed(); // benchmarking end

    println!("{}", format!(" Solution: {} ", resolution).on_green());
    println!("Elapsed: {}", format!(" {:.2?} ", done).on_blue());
}
