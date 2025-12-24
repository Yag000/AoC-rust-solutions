use aoc_star::star;

fn max_joltage(adapters: &Vec<u32>) -> u32 {
    let mut max = 0;
    let mut max_after_max = 0;
    for (i, battery) in adapters.iter().enumerate() {
        if *battery > max && i < (adapters.len() - 1) {
            max_after_max = 0;
            max = *battery;
            continue;
        }
        if *battery > max_after_max {
            max_after_max = *battery;
        }
    }
    (format!("{}{}", max, max_after_max)).parse().unwrap()
}

#[star(day = 3, part = 1)]
fn part1(input: String) -> String {
    input
        .lines()
        .fold(0, |acc, line| {
            let adapters: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
            acc + max_joltage(&adapters)
        })
        .to_string()
}

fn max_joltage2(adapters: &Vec<u64>) -> u64 {
    let mut current = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut current_index = 0;
    for (i, battery) in adapters.iter().enumerate() {
        for j in 0..=current_index {
            if adapters.len() - i >= 12 - j && *battery > current[j] {
                current[j] = *battery;
                for k in (j + 1)..12 {
                    current[k] = 0;
                }
                if j == current_index && current_index < 11 {
                    current_index += 1;
                }
                break;
            }
        }
    }

    let result_string: String = current.iter().map(|d| d.to_string()).collect();
    result_string.parse().unwrap()
}

#[star(day = 3, part = 2)]
fn part2(input: String) -> String {
    input
        .lines()
        .fold(0, |acc, line| {
            let adapters: Vec<u64> = line
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect();
            acc + max_joltage2(&adapters)
        })
        .to_string()
}
