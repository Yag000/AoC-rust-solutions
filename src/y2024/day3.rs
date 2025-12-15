use aoc_star::star;
use regex::Regex;

#[star(day = 3, part = 1, year = 2024)]
pub fn part1(input: String) -> String {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    re.captures_iter(input.as_str())
        .fold(0, |acc, line| {
            let x: u32 = line[1].parse().unwrap();
            let y: u32 = line[2].parse().unwrap();
            acc + (x * y)
        })
        .to_string()
}

#[star(day = 3, part = 2, year = 2024)]
fn part2(input: String) -> String {
    let re = Regex::new(r"(mul\((\d+),(\d+)\))|(do\(\))|(don't\(\))").unwrap();
    let (_, v) = re
        .captures_iter(input.as_str())
        .fold((true, 0), |(active, acc), cap| {
            if cap.get(1).is_some() {
                let acc = if active {
                    let x: u32 = cap[2].parse().unwrap();
                    let y: u32 = cap[3].parse().unwrap();
                    acc + (x * y)
                } else {
                    acc
                };
                (active, acc)
            } else if cap.get(4).is_some() {
                (true, acc)
            } else if cap.get(5).is_some() {
                (false, acc)
            } else {
                unreachable!()
            }
        });

    v.to_string()
}
