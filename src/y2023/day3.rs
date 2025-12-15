use crate::y2023::utils::adjacents;
use aoc_star::star;
use std::collections::HashSet;

#[star(day = 3, part = 1, year = 2023)]
fn part1(input: String) -> String {
    let mut board = vec![];

    for line in input.lines() {
        let mut line_vec = vec![];
        let mut i = 0;
        while i < line.len() {
            let c = line.chars().nth(i).unwrap();
            match c {
                '.' => {
                    line_vec.push(0);
                    i += 1;
                }
                x if x.is_ascii_digit() => {
                    let mut number = String::new();
                    number.push(c);
                    let mut k = 1;
                    while i + k < line.len() && line.chars().nth(i + k).unwrap().is_ascii_digit() {
                        number.push(line.chars().nth(i + k).unwrap());
                        k += 1;
                    }
                    for _ in 0..k {
                        line_vec.push(number.parse::<i32>().unwrap());
                    }
                    i += k;
                }
                _ => {
                    line_vec.push(-1);
                    i += 1;
                }
            }
        }

        board.push(line_vec);
    }
    count_numbers(&mut board).to_string()
}

fn count_numbers(input: &mut Vec<Vec<i32>>) -> i32 {
    let mut count = 0;
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if input[i][j] > 0 {
                for (x, y) in adjacents(i, j, input.len(), input[i].len()) {
                    if input[x][y] == -1 {
                        count += input[i][j];
                        let mut k = 0;
                        let initilal = input[i][j];
                        while j + k < input[i].len() && input[i][j + k] == initilal {
                            input[i][j + k] = 0;
                            k += 1;
                        }
                        break;
                    }
                }
            }
        }
    }
    count
}

#[star(day = 3, part = 2, year = 2023)]
fn part2(input: String) -> String {
    let mut board = vec![];

    for line in input.lines() {
        let mut line_vec = vec![];
        let mut i = 0;
        while i < line.len() {
            let c = line.chars().nth(i).unwrap();
            match c {
                '*' => {
                    line_vec.push(-1);
                    i += 1;
                }
                x if x.is_ascii_digit() => {
                    let mut number = String::new();
                    number.push(c);
                    let mut k = 1;
                    while i + k < line.len() && line.chars().nth(i + k).unwrap().is_ascii_digit() {
                        number.push(line.chars().nth(i + k).unwrap());
                        k += 1;
                    }
                    for _ in 0..k {
                        line_vec.push(number.parse::<i32>().unwrap());
                    }
                    i += k;
                }
                _ => {
                    line_vec.push(0);
                    i += 1;
                }
            }
        }

        board.push(line_vec);
    }

    count_numbers2(&mut board).to_string()
}

fn count_numbers2(input: &mut Vec<Vec<i32>>) -> i32 {
    let mut count = 0;
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if input[i][j] == -1 {
                let mut found = HashSet::new();
                for (x, y) in adjacents(i, j, input.len(), input[i].len()) {
                    if input[x][y] > 0 {
                        found.insert(input[x][y]);
                    }
                }
                if found.len() == 2 {
                    let found = found.iter().collect::<Vec<&i32>>();
                    count += found[0] * found[1];
                }
            }
        }
    }
    count
}
