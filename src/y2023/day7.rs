use aoc_star::star;

use std::{
    char,
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

#[star(day = 7, part = 1, year = 2023)]
fn part1(input: String) -> String {
    find_score(input, false)
}

#[star(day = 7, part = 2, year = 2023)]
fn part2(input: String) -> String {
    find_score(input, true)
}

fn find_score(input: String, is_joker: bool) -> String {
    let mut values = input
        .lines()
        .map(|line| {
            let mut parts = line.split(' ');
            (
                parts.next().unwrap(),
                parts.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    values.sort_by(|a, b| compare_hands(a.0, b.0, is_joker));

    values
        .iter()
        .enumerate()
        .fold(0, |acc, (i, (_, v))| acc + (i + 1) * *v)
        .to_string()
}

fn compare_hands(a: &str, b: &str, is_joker: bool) -> Ordering {
    let points = hand_points(a, is_joker).cmp(&hand_points(b, is_joker));
    if points != Ordering::Equal {
        return points;
    }

    for (a, b) in a.chars().zip(b.chars()) {
        let points = comapre_cards(a, b, is_joker);
        if points != Ordering::Equal {
            return points;
        }
    }

    Ordering::Equal
}

fn comapre_cards(a: char, b: char, is_joker: bool) -> Ordering {
    let a = card_value(a, is_joker);
    let b = card_value(b, is_joker);
    a.cmp(&b)
}

fn card_value(a: char, is_joker: bool) -> u32 {
    match a {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => {
            if is_joker {
                1
            } else {
                11
            }
        }
        'T' => 10,
        _ => a.to_digit(10).unwrap(),
    }
}

fn hand_points(hand: &str, is_joker: bool) -> u32 {
    let values = hand.chars().collect::<HashSet<char>>();
    let size = values.len();
    match size {
        1 => 6, // 5 of a kind
        2 => {
            let mut occurences = HashMap::new();
            for c in hand.chars() {
                let count = occurences.entry(c).or_insert(0);
                *count += 1;
            }

            for (_, v) in occurences.iter() {
                if *v == 4 {
                    // 4 of a kind or 1 of a kind
                    if is_joker && hand.contains('J') {
                        return 6;
                    }
                    return 5;
                }
            }

            // Full house
            if is_joker && hand.contains('J') {
                6 // There are 2 J's
            } else {
                4
            }
        }
        3 => {
            let mut occurences = HashMap::new();
            for c in hand.chars() {
                let count = occurences.entry(c).or_insert(0);
                *count += 1;
            }
            for (_, v) in occurences.clone().iter() {
                if *v == 3 {
                    // 3 of a kind, so 2 are different
                    if is_joker && hand.contains('J') {
                        // Only one J -> 4 of a kind
                        return 5;
                    }
                    return 3;
                }
            }
            if !is_joker {
                // 2 pairs
                return 2;
            }
            match occurences.get(&'J') {
                // 2 J's and 1 pairs -> 4 of a kind
                Some(2) => 5,
                // 1 J's and 1 pairs -> 3 of a kind
                Some(1) => 4,
                // 2 pairs, no J's
                _ => 2,
            }
        }
        4 => {
            if is_joker && hand.contains('J') {
                // 1 pair + 1 J or 2 J's -> 3 of a kind
                3
            } else {
                // 1 pairs
                1
            }
        }
        5 => {
            if is_joker && hand.contains('J') {
                // 1 J -> 1 pair
                1
            } else {
                // 0 J's
                0
            }
        }
        _ => panic!("Invalid hand"),
    }
}
