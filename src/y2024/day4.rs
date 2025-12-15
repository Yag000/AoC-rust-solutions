use aoc_star::star;

fn is_valid(t: &[Vec<char>], y: i32, x: i32) -> bool {
    y >= 0 && y < t.len() as i32 && x >= 0 && x < t[y as usize].len() as i32
}

fn check(t: &[Vec<char>], pos: (i32, i32), d: (i32, i32)) -> bool {
    let (x, y) = pos;
    let (dx, dy) = d;

    "MAS".chars().enumerate().all(|(i, c)| {
        let i = i as i32;

        is_valid(t, y + dy * (1 + i), x + dx * (1 + i))
            && t[(y + dy * (1 + i)) as usize][(x + dx * (1 + i)) as usize] == c
    })
}

#[star(day = 4, part = 1, year = 2024)]
fn part1(input: String) -> String {
    let t: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut counter = 0;

    for y in 0..t.len() {
        let y = y as i32;
        for x in 0..t[y as usize].len() {
            let x = x as i32;

            if t[y as usize][x as usize] == 'X' {
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        if check(&t, (x, y), (dx, dy)) {
                            counter += 1;
                        }
                    }
                }
            }
        }
    }

    counter.to_string()
}

fn check2(t: &[Vec<char>], x: i32, y: i32) -> bool {
    let vectors = [(1, 1), (-1, 1), (1, -1), (-1, -1)];

    if !vectors.iter().all(|(dx, dy)| is_valid(t, x + dx, y + dy)) {
        return false;
    }

    let (m, s) = vectors.iter().fold((0, 0), |(m, s), (dx, dy)| {
        let c = t[(y + dy) as usize][(x + dx) as usize];
        if c == 'M' {
            (m + 1, s)
        } else if c == 'S' {
            (m, s + 1)
        } else {
            (m, s)
        }
    });

    m == 2
        && s == 2
        && t[(y + 1) as usize][(x + 1) as usize] != t[(y - 1) as usize][(x - 1) as usize]
}

#[star(day = 4, part = 2, year = 2024)]
fn part2(input: String) -> String {
    let t: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut counter = 0;

    for y in 0..t.len() {
        for x in 0..t[y].len() {
            if t[y][x] == 'A' {
                let y = y as i32;
                let x = x as i32;
                if check2(&t, x, y) {
                    counter += 1;
                }
            }
        }
    }

    counter.to_string()
}
