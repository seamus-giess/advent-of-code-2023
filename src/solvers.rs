use colored::Colorize;

pub mod day_1_part_1;
pub mod day_1_part_2;
pub mod day_2_part_1;
pub mod day_2_part_2;
pub mod day_3_part_1;
pub mod day_3_part_2;

pub fn solve(day: &String, part: &String, data: &String) -> String {
    match format!("{},{}", day, part).as_str() {
        "1,1" => return day_1_part_1::solve(data),
        "1,2" => return day_1_part_2::solve(data),
        "2,1" => return day_2_part_1::solve(data),
        "2,2" => return day_2_part_2::solve(data),
        "3,1" => return day_3_part_1::solve(data),
        "3,2" => return day_3_part_2::solve(data),
        _ => {
            println!(
                "Solver {} not found!\nExiting...",
                format!("src/solvers/day_{}_part_{}.rs", day, part).red(),
            );
            std::process::exit(1);
        }
    }
}
