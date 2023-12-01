pub fn solve(data: &String) -> String {
    let letterless = data.replace(
        &[
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
            'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
            's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ][..],
        "",
    );

    let exploded = letterless.split("\n");

    let mut sum: u32 = 0;
    exploded.for_each(|x| {
        let first =
            x.chars().next().unwrap().to_digit(10).unwrap();
        let last =
            x.chars().last().unwrap().to_digit(10).unwrap();
        sum += (first * 10) + last;
    });

    return sum.to_string();
}
