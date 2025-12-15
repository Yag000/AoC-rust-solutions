use aoc_star::star;
use std::{cmp::Ordering, collections::HashMap};

fn update_hash(h: &mut HashMap<i32, Vec<i32>>, s: &str) {
    let before: i32 = s[0..2].parse().unwrap();
    let after: i32 = s[3..].parse().unwrap();

    match h.get_mut(&after) {
        None => {
            h.insert(after, vec![before]);
        }
        Some(v) => v.push(before),
    };
}

fn middle(t: &[i32]) -> i32 {
    let m = t.len() / 2;
    t[m]
}

fn is_valid(h: &HashMap<i32, Vec<i32>>, v: &[i32]) -> bool {
    for i in 0..v.len() {
        if let Some(cons) = h.get(&v[i]) {
            for value in v {
                if cons.contains(value) {
                    return false;
                }
            }
        }
    }

    true
}

#[star(day = 5, part = 1, year = 2024)]
fn part1(input: String) -> String {
    let mut flag = true;
    let mut h: HashMap<i32, Vec<i32>> = HashMap::new();

    let mut constraints: Vec<Vec<i32>> = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            flag = false;
            continue;
        }

        if flag {
            update_hash(&mut h, line);
        } else {
            constraints.push(line.split(',').map(|v| v.parse().unwrap()).collect());
        }
    }

    constraints
        .iter()
        .filter_map(|v| {
            if is_valid(&h, v) {
                Some(middle(v))
            } else {
                None
            }
        })
        .sum::<i32>()
        .to_string()
}

fn sort_line(h: &HashMap<i32, Vec<i32>>, v: &[i32]) -> Vec<i32> {
    let mut clone = v.to_owned();
    clone.sort_by(|a, b| match h.get(a) {
        Some(v) => {
            if v.contains(b) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }
        None => Ordering::Equal,
    });
    clone
}

#[star(day = 5, part = 2, year = 2024)]
fn part2(input: String) -> String {
    let mut flag = true;
    let mut h: HashMap<i32, Vec<i32>> = HashMap::new();

    let mut constraints: Vec<Vec<i32>> = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            flag = false;
            continue;
        }

        if flag {
            update_hash(&mut h, line);
        } else {
            constraints.push(line.split(',').map(|v| v.parse().unwrap()).collect());
        }
    }

    constraints
        .iter()
        .filter_map(|v| {
            if is_valid(&h, v) {
                None
            } else {
                let v_sorted = sort_line(&h, v);
                Some(middle(&v_sorted))
            }
        })
        .sum::<i32>()
        .to_string()
}
