use aoc_star::star;
use std::collections::HashSet;

#[star(day = 4, part = 1, year = 2023)]
fn part1(input: String) -> String {
    input
        .lines()
        .map(|line| {
            let mut values = line.split('|').collect::<Vec<&str>>();
            values[0] = values[0].split(':').collect::<Vec<&str>>()[1];
            let left = values[0]
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<u32>().expect("oops"))
                .collect::<HashSet<u32>>();

            values[1]
                .split(' ')
                .filter(|s| !s.is_empty())
                .fold(0, |acc, s| {
                    if left.contains(&s.parse::<u32>().expect("oops2")) {
                        if acc == 0 { 1 } else { acc * 2 }
                    } else {
                        acc
                    }
                })
        })
        .sum::<u32>()
        .to_string()
}

#[star(day = 4, part = 2, year = 2023)]
fn part2(input: String) -> String {
    let mut file = vec![];
    input
        .lines()
        .fold(0, |acc, line| {
            let mut values = line.split('|').collect::<Vec<&str>>();
            values[0] = values[0].split(':').collect::<Vec<&str>>()[1];

            let left = values[0]
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<u32>().expect("oops"))
                .collect::<HashSet<u32>>();

            let winners = values[1]
                .split(' ')
                .filter(|s| !s.is_empty() && left.contains(&s.parse::<u32>().expect("oops2")))
                .count();

            let copies = if file.is_empty() {
                1
            } else {
                file.remove(0) + 1
            };
            let len_stack = file.len();
            for i in 0..winners {
                if i >= len_stack {
                    file.push(copies)
                } else {
                    file[i] += copies
                }
            }
            acc + copies
        })
        .to_string()
}
