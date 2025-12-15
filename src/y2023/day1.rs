use aoc_star::star;

#[star(day = 1, part = 1, year = 2023)]
fn part1(input: String) -> String {
    input
        .lines()
        .map(|s| {
            let (a, b) = get_first_and_last_digit(s);
            format!("{}{}", a, b).parse::<u32>().unwrap()
        })
        .sum::<u32>()
        .to_string()
}

fn get_first_and_last_digit(str: &str) -> (char, char) {
    let first = str.chars().find(|c| c.is_ascii_digit()).unwrap();
    let last = str.chars().rev().find(|c| c.is_ascii_digit()).unwrap();
    (first, last)
}

#[star(day = 1, part = 2, year = 2023)]
fn part2(input: String) -> String {
    input
        .lines()
        .map(|s| {
            let (a, b) = get_first_and_last_digit_with_stings(s);
            format!("{}{}", a, b).parse::<u32>().unwrap()
        })
        .sum::<u32>()
        .to_string()
}

const NUMBERS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn get_first_and_last_digit_with_stings(str: &str) -> (u32, u32) {
    let mut digits: Vec<u32> = Vec::new();
    for (i, w) in str.chars().enumerate() {
        if w.is_ascii_digit() {
            digits.push(w.to_digit(10).unwrap());
        }
        for (j, n) in NUMBERS.iter().enumerate() {
            if str[i..].starts_with(n) {
                digits.push(j as u32);
            }
        }
    }
    (*digits.first().unwrap(), *digits.last().unwrap())
}
