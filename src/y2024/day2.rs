use aoc_star::star;

enum Order {
    Nope,
    Up,
    Down,
}

#[star(day = 2, part = 1, year = 2024)]
fn part1(input: String) -> String {
    let sum: i32 = input
        .lines()
        .map(|line| {
            let numbers: Vec<i32> = line
                .split_whitespace()
                .map(|l| l.parse().unwrap())
                .collect();

            let mut last = numbers[0];

            let mut dec = Order::Nope;

            let mut flag = true;

            for n in numbers[1..].iter() {
                let v = last - n;
                match dec {
                    Order::Nope => {
                        if v > 0 && v < 4 {
                            dec = Order::Down;
                        } else if v < 0 && v > -4 {
                            dec = Order::Up
                        } else {
                            flag = false;
                            break;
                        }
                    }
                    Order::Down => {
                        if !(v > 0 && v < 4) {
                            flag = false;
                            break;
                        }
                    }

                    Order::Up => {
                        if !(v < 0 && v > -4) {
                            flag = false;
                            break;
                        }
                    }
                }
                last = *n;
            }

            if flag { 1 } else { 0 }
        })
        .sum();

    sum.to_string()
}

fn count_errors(numbers: &[i32], first: i32, starting_index: usize) -> i32 {
    let mut last = first;

    let mut dec = Order::Nope;

    let mut errors = 0;

    for n in numbers[starting_index..].iter() {
        let v = last - n;
        match dec {
            Order::Nope => {
                if v > 0 && v < 4 {
                    dec = Order::Down;
                } else if v < 0 && v > -4 {
                    dec = Order::Up
                } else {
                    errors += 1;
                    continue;
                }
            }
            Order::Down => {
                if !(v > 0 && v < 4) {
                    errors += 1;
                    continue;
                }
            }

            Order::Up => {
                if !(v < 0 && v > -4) {
                    errors += 1;
                    continue;
                }
            }
        }
        last = *n;
    }
    errors
}

#[star(day = 2, part = 2, year = 2024)]
fn part2(input: String) -> String {
    let sum: i32 = input
        .lines()
        .map(|line| {
            let numbers: Vec<i32> = line
                .split_whitespace()
                .map(|l| l.parse().unwrap())
                .collect();

            let errors = count_errors(&numbers, numbers[0], 1);

            if errors < 2 {
                1
            } else {
                let errors = count_errors(&numbers, numbers[1], 2);
                if errors == 0 { 1 } else { 0 }
            }
        })
        .sum();

    sum.to_string()
}
