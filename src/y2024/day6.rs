use aoc_star::star;
use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn turn(d: Direction) -> Direction {
    match d {
        Direction::Up => Direction::Right,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
        Direction::Right => Direction::Down,
    }
}

fn vector_of_dir(d: &Direction) -> (i32, i32) {
    match d {
        Direction::Up => (-1, 0),
        Direction::Down => (1, 0),
        Direction::Left => (0, -1),
        Direction::Right => (0, 1),
    }
}

fn next_pos(p: (i32, i32), d: &Direction) -> (i32, i32) {
    let (x, y) = p;
    let (dx, dy) = vector_of_dir(d);

    (x + dx, y + dy)
}

fn is_inside(t: &[Vec<Option<bool>>], p: (i32, i32)) -> bool {
    let (x, y) = p;
    y >= 0 && y < t.len() as i32 && x >= 0 && x < t[y as usize].len() as i32
}

fn is_wall(t: &[Vec<Option<bool>>], p: (i32, i32)) -> bool {
    let (x, y) = p;
    is_inside(t, p) && t[x as usize][y as usize].is_none()
}

#[star(day = 6, part = 1, year = 2024)]
fn part1(input: String) -> String {
    let mut board: Vec<Vec<Option<bool>>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    if c == '.' {
                        Some(false)
                    } else if c == '#' {
                        None
                    } else if c == '^' {
                        Some(true)
                    } else {
                        unreachable!()
                    }
                })
                .collect()
        })
        .collect();

    let mut pos = (0, 0);

    for i in 0..board.len() {
        for j in 0..board[0].len() {
            if board[i][j] == Some(true) {
                pos = (i as i32, j as i32);
                break;
            }
        }
    }

    let mut d = Direction::Up;

    while is_inside(&board, pos) {
        let n = next_pos(pos, &d);
        if is_wall(&board, n) {
            d = turn(d);
        } else {
            board[pos.0 as usize][pos.1 as usize] = Some(true);
            pos = n;
        }
    }

    board
        .iter()
        .flatten()
        .filter_map(|v| match v {
            Some(true) => Some(1),
            _ => None,
        })
        .sum::<i32>()
        .to_string()
}

fn update(h: &mut HashMap<(i32, i32), Vec<Direction>>, d: Direction, p: (i32, i32)) {
    match h.get_mut(&p) {
        None => {
            h.insert(p, vec![d]);
        }
        Some(s) => s.push(d),
    }
}

fn loops(t: &[Vec<Option<bool>>], p: (i32, i32)) -> bool {
    let mut pos = p;

    let mut d = Direction::Up;

    let mut board = t.to_owned();

    let mut h: HashMap<(i32, i32), Vec<Direction>> = HashMap::new();

    while is_inside(&board, pos) {
        let n = next_pos(pos, &d);

        if board[pos.0 as usize][pos.1 as usize] == Some(true) {
            if let Some(v) = h.get(&pos) {
                if v.contains(&d) {
                    return true;
                }
            }
        }

        if is_wall(&board, n) {
            d = turn(d);
        } else {
            board[pos.0 as usize][pos.1 as usize] = Some(true);
            update(&mut h, d, pos);
            pos = n;
        }
    }

    false
}

#[star(day = 6, part = 2, year = 2024)]
fn part2(input: String) -> String {
    let mut board: Vec<Vec<Option<bool>>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    if c == '.' {
                        Some(false)
                    } else if c == '#' {
                        None
                    } else if c == '^' {
                        Some(true)
                    } else {
                        unreachable!()
                    }
                })
                .collect()
        })
        .collect();

    let mut pos = (0, 0);

    for i in 0..board.len() {
        for j in 0..board[0].len() {
            if board[i][j] == Some(true) {
                pos = (i as i32, j as i32);
                break;
            }
        }
    }

    let mut counter = 0;
    for i in 0..board.len() {
        for j in 0..board[0].len() {
            if board[i][j] == Some(false) {
                board[i][j] = None;
                if loops(&board, pos) {
                    counter += 1;
                }
                board[i][j] = Some(false);
            }
        }
    }

    counter.to_string()
}
