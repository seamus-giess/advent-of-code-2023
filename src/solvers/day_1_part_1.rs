use regex::Regex;

pub fn solve(data: &String) -> String {
    let reg_ex: Regex = match Regex::new(r"[^\d]") {
        Ok(reg_ex) => reg_ex,
        _ => panic!("Regex string is malformed."),
    };
    let lines = data.split("\n");
    let letterless: Vec<String> = lines
        .map(|line| {
            return reg_ex.replace_all(line, "").to_string();
        })
        .collect();

    let mut sum: u32 = 0;
    letterless.into_iter().for_each(|line| {
        let first = match line.chars().next() {
            Some(char) => match char.to_digit(10) {
                Some(digit) => digit,
                None => panic!("Could not parse first char into digit."),
            },
            None => panic!("Could not extract first char from line."),
        };
        let last = match line.chars().last() {
            Some(char) => match char.to_digit(10) {
                Some(digit) => digit,
                None => panic!("Could not parse first char into digit."),
            },
            None => panic!("Could not extract first char from line."),
        };
        sum += (first * 10) + last;
    });

    return sum.to_string();
}
