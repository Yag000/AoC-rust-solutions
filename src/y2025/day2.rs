use aoc_star::star;

#[star(day = 2, part = 1)]
fn solve_part1(input: String) -> String {
    input
        .trim()
        .split(',')
        .fold(0, |acc, interval| {
            let mut bounds = interval.split('-');
            let start: usize = bounds.next().unwrap().parse().unwrap();
            let end: usize = bounds.next().unwrap().parse().unwrap();
            acc + handle_interval(start, end, 1)
        })
        .to_string()
}

#[star(day = 2, part = 2)]
fn solve_part2(input: String) -> String {
    input
        .trim()
        .split(',')
        .fold(0, |acc, interval| {
            let mut bounds = interval.split('-');
            let start: usize = bounds.next().unwrap().parse().unwrap();
            let end: usize = bounds.next().unwrap().parse().unwrap();
            acc + handle_interval(start, end, 2)
        })
        .to_string()
}

fn handle_interval(start: usize, end: usize, part: u8) -> usize {
    let mut count = 0;
    for i in start..=end {
        let s = i.to_string();
        match part {
            1 => {
                if !is_double_number(&s) {
                    count += i;
                }
            }
            2 => {
                if !is_valid(&s) {
                    count += i;
                }
            }
            _ => {}
        }
    }
    count
}

fn is_double_number(s: &str) -> bool {
    if !s.len().is_multiple_of(2) {
        return true;
    }

    let half = s.len() / 2;
    let (first_half, second_half) = s.split_at(half);
    if first_half != second_half {
        return true;
    }

    false
}

fn is_valid(s: &str) -> bool {
    // We only need to check half of the word for a prefix
    let half = s.len() / 2;
    for i in 1..=half {
        let prefix = &s[0..i];

        // If the length of s is not a multiple of the prefix length, skip
        if !s.len().is_multiple_of(prefix.len()) {
            continue;
        }

        // Check if the prefix repeats to form the entire string
        for j in (0..s.len()).step_by(prefix.len()) {
            // If any part does not match the prefix, break
            if &s[j..j + prefix.len()] != prefix {
                break;
            }

            // If we reached the end of the string and all parts matched the prefix
            if j + prefix.len() == s.len() {
                return false;
            }
        }
    }

    true
}
