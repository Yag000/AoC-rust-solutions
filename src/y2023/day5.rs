use aoc_star::star;

#[star(day = 5, part = 1, year = 2023)]
fn part1(input: String) -> String {
    let (mut mapped, unmapped) =
        input
            .lines()
            .fold((vec![], vec![]), |(mut mapped, mut unmapped), line| {
                if line.starts_with("seeds:") {
                    let seeds = line.split(':').collect::<Vec<&str>>()[1]
                        .split(' ')
                        .filter(|s| !s.is_empty())
                        .map(|s| s.trim().parse::<u64>().unwrap())
                        .collect::<Vec<u64>>();
                    (mapped, seeds)
                } else if line.is_empty() {
                    (mapped, unmapped)
                } else if line.chars().collect::<Vec<char>>()[0].is_alphabetic() {
                    mapped.append(&mut unmapped.clone());
                    (vec![], mapped)
                } else {
                    let range = line
                        .split(' ')
                        .map(|s| s.trim().parse::<u64>().unwrap())
                        .collect::<Vec<u64>>();
                    let mut to_remove = vec![];
                    for (i, unmapped_elt) in unmapped.iter().enumerate() {
                        if *unmapped_elt >= range[1] && *unmapped_elt < range[1] + range[2] {
                            mapped.push(range[0] + unmapped_elt - range[1]);
                            to_remove.push(i);
                        }
                    }
                    for i in to_remove.iter().rev() {
                        unmapped.remove(*i);
                    }
                    (mapped, unmapped)
                }
            });
    mapped.append(&mut unmapped.clone());
    mapped.iter().min().unwrap().to_string()
}

#[star(day = 5, part = 2, year = 2023)]
fn part2(input: String) -> String {
    let mut initial_seed_ranges = vec![];
    let mut maps = vec![];
    input.lines().for_each(|line| {
        if line.starts_with("seeds:") {
            let seeds = line.split(':').collect::<Vec<&str>>()[1]
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|s| s.trim().parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            for i in 0..seeds.len() / 2 {
                initial_seed_ranges.push((seeds[2 * i], seeds[2 * i + 1]));
            }
        } else if !line.is_empty() {
            if line.chars().collect::<Vec<char>>()[0].is_alphabetic() {
                maps.push(vec![]);
            } else {
                let range = line
                    .split(' ')
                    .map(|s| s.trim().parse::<u64>().unwrap())
                    .collect::<Vec<u64>>();
                let maps_len = maps.len();
                maps[maps_len - 1].push(range);
            }
        }
    });

    let maps = maps.iter().rev().collect::<Vec<&Vec<Vec<u64>>>>();

    let mut last_values = maps[0].clone();
    last_values.sort_by(|a, b| a[0].cmp(&b[0]));

    if last_values[0][0] != 0 {
        last_values.insert(0, vec![0, 0, last_values[0][0]]);
    }

    for last_value in last_values {
        for i in 0..last_value[2] {
            let mut element = i + last_value[0] + last_value[1];
            for map in maps.iter().skip(1) {
                for range in map.iter() {
                    if element >= range[0] && element < range[0] + range[2] {
                        element = range[1] + element - range[0];
                        break;
                    }
                }
            }
            for seed_range in initial_seed_ranges.iter() {
                if element >= seed_range.0 && element < seed_range.0 + seed_range.1 {
                    return (i + last_value[0]).to_string();
                }
            }
        }
    }

    panic!("No seed found")
}
